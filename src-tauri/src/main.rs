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
    return ("".to_string(), "".to_string());
}

#[tauri::command]
fn review_card(deck_name: String, card_question: String, difficulty: String) -> Result<(), String> {
    println!("r1 {}", difficulty);
    let app = APP.lock().unwrap();
    let difficulty_enum = match difficulty.as_str() {
        "wrong" => review::ReviewDifficulty::Hard,
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
fn get_deck_names() -> Vec<Deck> {
    let app = APP.lock().unwrap();
    match app.load_decks() {
        Ok(decks) => decks,
        Err(_) => Vec::new(),
    }
}

#[tauri::command]
fn add_deck(deck_name: String) -> Result<(), String> {
    println!("h1");
    let app = APP.lock().unwrap();
    match app.add_deck(deck_name) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn add_card(deck_name: String, question: String, answer: String) -> Result<(), String> {
    println!("h2");
    let app = APP.lock().unwrap();
    match app.add_card(deck_name, question, answer) {
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
                name TEXT PRIMARY KEY
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

    pub fn add_deck(&self, deck_name: String) -> Result<()> {
        self.conn
            .execute("INSERT INTO decks (name) VALUES (?1)", params![deck_name])?;

        Ok(())
    }

    pub fn add_card(&self, deck_name: String, question: String, answer: String) -> Result<()> {
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
                (Utc::now() + Duration::days(1)).timestamp(),
                Duration::days(1).num_seconds(),
                2.5,
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

    pub fn load_decks(&self) -> Result<Vec<Deck>> {
        let mut stmt = self.conn.prepare("SELECT name FROM decks")?;
        let deck_names = stmt.query_map([], |row| row.get(0))?;

        let mut decks = Vec::new();
        for deck_name in deck_names {
            let deck_name = deck_name?;
            let mut stmt = self.conn.prepare(
                "SELECT question, answer, next_review_at, interval, ease_factor, reviews_count, successful_reviews, failed_reviews FROM cards WHERE deck_name = ?1"
            )?;
            let cards = stmt.query_map(params![deck_name], |row| {
                Ok(Card {
                    question: row.get(0)?,
                    answer: row.get(1)?,
                    schedule: ReviewSchedule {
                        next_review_at: row.get(2)?, // Datum der nächsten Überprüfung aus der Datenbank
                        interval: row.get(3)?,       // Intervall aus der Datenbank
                        ease_factor: row.get(4)?,    // Schwierigkeitsgrad aus der Datenbank
                        reviews_count: row.get(5)?,  // Anzahl der Überprüfungen aus der Datenbank
                        successful_reviews: row.get(6)?, // Anzahl der erfolgreichen Überprüfungen aus der Datenbank
                        failed_reviews: row.get(7)?, // Anzahl der gescheiterten Überprüfungen aus der Datenbank
                    },
                })
            })?;

            let mut card_vec = Vec::new();
            for card in cards {
                card_vec.push(card?);
            }

            decks.push(Deck {
                name: deck_name,
                cards: card_vec,
            });
        }

        Ok(decks)
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_card,
            get_deck_names,
            add_deck,
            add_card,
            review_card
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
// TODO: ease factor und initial_interval für jedes Deck konfigurierbar machen
