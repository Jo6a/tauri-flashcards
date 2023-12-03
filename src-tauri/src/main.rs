// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod review;

use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;
use review::{ReviewDifficulty, ReviewSchedule};
use rusqlite::{params, Connection, Result};
use std::sync::Mutex;

lazy_static! {
    static ref APP: Mutex<App> = Mutex::new(App::new("database.db").unwrap());
}

#[derive(serde::Serialize)]
pub struct Card {
    question: String,
    answer: String,
    schedule: ReviewSchedule,
}

#[derive(serde::Serialize)]
pub struct Deck {
    name: String,
    initial_interval: i64,
    initial_ease_factor: f32,
    cards: Vec<Card>,
}

#[tauri::command]
fn get_card(deck_name: String) -> (String, String) {
    println!("get_card: {}", deck_name);
    let app = APP.lock().unwrap();
    let decks = match app.load_decks() {
        Ok(decks) => decks,
        Err(_) => Vec::new(),
    };
    for deck in decks.iter() {
        if deck.name == deck_name && deck.cards.len() > 0 {
            println!("get_card2: {}", deck.cards[0].question.clone());
            if let Some(oldest_card) = deck
                .cards
                .iter()
                .min_by_key(|card| card.schedule.next_review_at)
            {
                return (oldest_card.question.clone(), oldest_card.answer.clone());
            }

            //return (deck.cards[0].question.clone(), deck.cards[0].answer.clone());
        }
    }
    return ("Done".to_string(), "Done".to_string());
}

#[tauri::command]
fn review_card(deck_name: String, card_question: String, difficulty: String) -> Result<(), String> {
    println!("r1 {}", difficulty);
    let app = APP.lock().unwrap();
    let difficulty_enum = match difficulty.as_str() {
        "wrong" => review::ReviewDifficulty::Wrong,
        "hard" => review::ReviewDifficulty::Hard,
        "good" => review::ReviewDifficulty::Medium,
        "easy" => review::ReviewDifficulty::Easy,
        _ => return Err("Invalid difficulty level".to_string()),
    };

    let (deck_index, card_index) = {
        let decks = app.load_decks().map_err(|err| err.to_string())?;
        decks
            .iter()
            .enumerate()
            .find_map(|(di, deck)| {
                if deck.name == deck_name {
                    deck.cards
                        .iter()
                        .enumerate()
                        .find(|(_, card)| card.question == card_question)
                        .map(|(ci, _)| (di, ci))
                } else {
                    None
                }
            })
            .ok_or("Deck or card not found".to_string())?
    };

    let mut decks = app.load_decks().map_err(|err| err.to_string())?;
    let deck = &mut decks[deck_index];
    let card = &mut deck.cards[card_index];

    println!("r11 {}", card.schedule.reviews_count);
    card.schedule.review(difficulty_enum);
    println!("r12 {}", card.schedule.reviews_count);

    app.update_card_review_schedule(&deck.name, &card.question, &card.schedule)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn update_card(deck_name: String, old_card_question: String, new_card_question: String, card_answer: String, next_review_at : i64) -> Result<(), String> {
    let app = APP.lock().unwrap();
    app.update_card(&deck_name, &old_card_question, &new_card_question, &card_answer, next_review_at)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn get_deckoptions(deck_name: String) -> (i64, f32) {
    let app = APP.lock().unwrap();
    match app.get_deckoptions(deck_name) {
        Ok(params) => params,
        Err(_) => (0, 0.0),
    }
}

#[tauri::command]
fn set_deckoptions(
    deck_name: String,
    initial_interval: i64,
    initial_ease_factor: f32,
) -> Result<(), String> {
    println!("{} {} {}", deck_name, initial_interval, initial_ease_factor);
    let app = APP.lock().unwrap();
    match app.set_deckoptions(deck_name, initial_interval, initial_ease_factor) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn get_decks() -> Vec<Deck> {
    let app = APP.lock().unwrap();
    match app.load_decks() {
        Ok(decks) => decks,
        Err(_) => Vec::new(),
    }
}

#[tauri::command]
fn get_cards(deck_name: String) -> Vec<Card> {
    println!("get_cards1");
    let app = APP.lock().unwrap();
    match app.get_cards(deck_name) {
        Ok(cards) => cards,
        Err(_) => Vec::new(),
    }
}

#[tauri::command]
fn add_deck(
    deck_name: String,
    initial_interval: i64,
    initial_ease_factor: f32,
) -> Result<(), String> {
    println!("h1");
    let app = APP.lock().unwrap();
    match app.add_deck(deck_name, initial_interval, initial_ease_factor) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn delete_deck(deck_name: String) -> Result<(), String> {
    println!("e1");
    let app = APP.lock().unwrap();
    match app.delete_deck(deck_name) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn add_card(
    deck_name: String,
    question: String,
    answer: String,
    initial_interval: i64,
    initial_ease_factor: f32,
) -> Result<(), String> {
    println!("h2");
    let app = APP.lock().unwrap();
    match app.add_card(
        deck_name,
        question,
        answer,
        initial_interval,
        initial_ease_factor,
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn delete_card(deck_name: String, question: String) -> Result<(), String> {
    let app = APP.lock().unwrap();
    match app.delete_card(deck_name, question) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

pub struct App {
    conn: Connection,
}

impl App {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS decks (
                name TEXT PRIMARY KEY,
                initial_interval INTEGER,
                initial_ease_factor REAL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS cards (
                card_id INTEGER PRIMARY KEY,
                question TEXT,
                answer TEXT,
                next_review_at INTEGER,
                interval INTEGER,
                ease_factor REAL,
                reviews_count INTEGER,
                successful_reviews INTEGER,
                failed_reviews INTEGER,
                deck_name TEXT,
                FOREIGN KEY(deck_name) REFERENCES decks(name)
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn add_deck(
        &self,
        deck_name: String,
        initial_interval: i64,
        initial_ease_factor: f32,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT INTO decks (name, initial_interval, initial_ease_factor) VALUES (?1, ?2, ?3)",
            params![deck_name, initial_interval, initial_ease_factor],
        )?;

        Ok(())
    }

    pub fn delete_deck(&self, deck_name: String) -> Result<(), rusqlite::Error> {
        self.conn
            .execute("DELETE FROM cards WHERE deck_name = ?1", params![deck_name])?;

        self.conn
            .execute("DELETE FROM decks WHERE name = ?1", params![deck_name])?;

        Ok(())
    }

    pub fn add_card(
        &self,
        deck_name: String,
        question: String,
        answer: String,
        initial_interval: i64,
        initial_ease_factor: f32,
    ) -> Result<()> {
        println!("add_card1: {} {} {}", deck_name, question, answer);

        let result = self.conn.execute(
            r#"
            INSERT INTO cards (
                question,
                answer,
                next_review_at,
                interval,
                ease_factor,
                reviews_count,
                successful_reviews,
                failed_reviews,
                deck_name
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
            params![
                question,
                answer,
                Utc::now().timestamp(),
                Duration::hours(initial_interval).num_seconds(),
                initial_ease_factor,
                0,
                0,
                0,
                deck_name
            ],
        );

        match result {
            Ok(rows_affected) => {
                println!(
                    "Successfully inserted card. Rows affected: {}",
                    rows_affected
                );
                Ok(())
            }
            Err(err) => {
                eprintln!("Failed to insert card: {}", err);
                Err(err)
            }
        }
    }

    pub fn delete_card(&self, deck_name: String, question: String) -> Result<(), rusqlite::Error> {
        self.conn
            .execute("DELETE FROM cards WHERE deck_name = ?1 AND question = ?2", params![deck_name, question])?;

        Ok(())
    }

    pub fn load_decks(&self) -> Result<Vec<Deck>, rusqlite::Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, initial_interval, initial_ease_factor FROM decks")?;
        let mut decks = Vec::new();

        let deck_data = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;

        for deck_tuple in deck_data {
            let (deck_name, initial_interval, initial_ease_factor) = deck_tuple?;

            let mut stmt = self.conn.prepare(
                "SELECT question, answer, next_review_at, interval, ease_factor, reviews_count, successful_reviews, failed_reviews FROM cards WHERE deck_name = ?1"
            )?;
            let cards = stmt.query_map(params![&deck_name], |row| {
                Ok(Card {
                    question: row.get(0)?,
                    answer: row.get(1)?,
                    schedule: ReviewSchedule {
                        next_review_at: row.get(2)?,
                        interval: row.get(3)?,
                        ease_factor: row.get(4)?,
                        reviews_count: row.get(5)?,
                        successful_reviews: row.get(6)?,
                        failed_reviews: row.get(7)?,
                    },
                })
            })?;

            let mut card_vec = Vec::new();
            for card in cards {
                card_vec.push(card?);
            }

            decks.push(Deck {
                name: deck_name,
                initial_interval,
                initial_ease_factor,
                cards: card_vec,
            });
        }

        Ok(decks)
    }

    pub fn get_cards(&self, deck_name: String) -> Result<Vec<Card>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT question, answer, next_review_at, interval, ease_factor, reviews_count, successful_reviews, failed_reviews FROM cards WHERE deck_name = ?1"
        )?;
        let cards = stmt.query_map(params![&deck_name], |row| {
            Ok(Card {
                question: row.get(0)?,
                answer: row.get(1)?,
                schedule: ReviewSchedule {
                    next_review_at: row.get(2)?,
                    interval: row.get(3)?,
                    ease_factor: row.get(4)?,
                    reviews_count: row.get(5)?,
                    successful_reviews: row.get(6)?,
                    failed_reviews: row.get(7)?,
                },
            })
        })?;

        let mut card_vec = Vec::new();
        for card in cards {
            card_vec.push(card?);
        }

        Ok(card_vec)
    }

    pub fn update_card_review_schedule(
        &self,
        deck_name: &str,
        card_question: &str,
        schedule: &ReviewSchedule,
    ) -> rusqlite::Result<()> {
        self.conn.execute(
            "UPDATE cards SET next_review_at = ?1, ease_factor = ?2, reviews_count = ?3, successful_reviews = ?4, failed_reviews = ?5 WHERE deck_name = ?6 AND question = ?7",
            rusqlite::params![
                schedule.next_review_at,
                schedule.ease_factor,
                schedule.reviews_count,
                schedule.successful_reviews,
                schedule.failed_reviews,
                deck_name,
                card_question,
            ],
        )?;

        Ok(())
    }

    pub fn update_card(
        &self,
        deck_name: &str,
        old_card_question: &str,
        new_card_question: &str,
        card_answer: &str,
        next_review_at: i64,
    ) -> rusqlite::Result<()> {

        println!("{} {} {} {} {}", deck_name, old_card_question, new_card_question, card_answer, next_review_at);
        let result = self.conn.execute(
            "UPDATE cards SET question = ?1, answer = ?2, next_review_at = ?3 WHERE deck_name = ?4 AND question = ?5",
            rusqlite::params![
                new_card_question,
                card_answer,
                next_review_at,
                deck_name,
                old_card_question
            ],
        );
        match result {
            Ok(rows_updated) => {
                println!("Rows updated: {}", rows_updated);
                if rows_updated == 0 {
                    println!("No rows were updated, check if the deck_name is correct and exists in the database.");
                }
                Ok(())
            }
            Err(e) => {
                println!("Error updating deck options: {:?}", e);
                Err(e)
            }
        }
    }

    pub fn get_deckoptions(&self, deck_name: String) -> Result<(i64, f32), rusqlite::Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT initial_interval, initial_ease_factor FROM decks WHERE name = ?1")?;
        let mut tuple_iter =
            stmt.query_map(params![&deck_name], |row| Ok((row.get(0)?, row.get(1)?)))?;

        if let Some(result) = tuple_iter.next() {
            result.map_err(Into::into)
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    pub fn set_deckoptions(
        &self,
        deck_name: String,
        initial_interval: i64,
        initial_ease_factor: f32,
    ) -> rusqlite::Result<()> {
        println!("Setting deck options for: {}", deck_name);
        println!(
            "Initial interval: {}, Initial ease factor: {}",
            initial_interval, initial_ease_factor
        );
        let result = self.conn.execute(
            "UPDATE decks SET initial_interval = ?1, initial_ease_factor = ?2 WHERE name = ?3",
            rusqlite::params![initial_interval, initial_ease_factor, deck_name],
        );
        match result {
            Ok(rows_updated) => {
                println!("Rows updated: {}", rows_updated);
                if rows_updated == 0 {
                    println!("No rows were updated, check if the deck_name is correct and exists in the database.");
                }
                Ok(())
            }
            Err(e) => {
                println!("Error updating deck options: {:?}", e);
                Err(e)
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_card,
            get_decks,
            get_cards,
            add_deck,
            delete_deck,
            add_card,
            delete_card,
            review_card,
            update_card,
            get_deckoptions,
            set_deckoptions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
