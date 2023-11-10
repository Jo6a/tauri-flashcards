// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use rusqlite::{params, Connection, Result};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref APP: Mutex<App> = Mutex::new(App::new("database.db").unwrap());
}

#[derive(serde::Serialize)]
pub struct Card {
    question: String,
    answer: String,
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
            return (deck.cards[0].question.clone(), deck.cards[0].answer.clone());
        }
    }
    return ("".to_string(), "".to_string());
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
                question TEXT,
                answer TEXT,
                deck_name TEXT,
                FOREIGN KEY(deck_name) REFERENCES decks(name)
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn add_deck(&self, deck_name: String) -> Result<()> {
        self.conn.execute(
            "INSERT INTO decks (name) VALUES (?1)",
            params![deck_name],
        )?;

        Ok(())
    }

    pub fn add_card(&self, deck_name: String, question: String, answer: String) -> Result<()> {
        println!("add_card1: {} {} {}", deck_name, question, answer);
        let result = self.conn.execute(
            "INSERT INTO cards (question, answer, deck_name) VALUES (?1, ?2, ?3)",
            params![question, answer, deck_name],
        );

        match result {
            Ok(rows_affected) => {
                println!("Successfully inserted card. Rows affected: {}", rows_affected);
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
            let mut stmt = self.conn.prepare("SELECT question, answer FROM cards WHERE deck_name = ?1")?;
            let cards = stmt.query_map(params![deck_name], |row| {
                Ok(Card {
                    question: row.get(0)?,
                    answer: row.get(1)?,
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

    // Weitere Methoden zum Hinzufügen/Bearbeiten/Löschen von Karten...
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_card, get_deck_names, add_deck, add_card])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

/*
1. **Deck erstellen**:
    - Wenn ein Benutzer ein neues Deck erstellt, senden Sie die Deck-Informationen (wie den Namen des Decks) vom Frontend an Rust.
    - Rust erstellt ein neues `Deck`-Objekt und fügt es zur Sammlung der Decks hinzu.

2. **Karte hinzufügen**:
    - Wenn ein Benutzer eine neue Karte zu einem Deck hinzufügt, senden Sie die Karteninformationen (Frage und Antwort) und den Namen des Decks vom Frontend an Rust.
    - Rust erstellt ein neues `Card`-Objekt und fügt es zum entsprechenden `Deck` hinzu.

3. **Karte bearbeiten**:
    - Wenn ein Benutzer eine Karte bearbeitet, senden Sie die aktualisierten Karteninformationen und den Namen des Decks vom Frontend an Rust.
    - Rust aktualisiert das entsprechende `Card`-Objekt im entsprechenden `Deck`.

4. **Karte löschen**:
    - Wenn ein Benutzer eine Karte löscht, senden Sie die Karteninformationen und den Namen des Decks vom Frontend an Rust.
    - Rust entfernt das entsprechende `Card`-Objekt aus dem entsprechenden `Deck`.

5. **Deck anzeigen**:
    - Wenn ein Benutzer ein Deck anzeigen möchte, senden Sie den Namen des Decks vom Frontend an Rust.
    - Rust sendet die Informationen des entsprechenden `Deck`-Objekts (einschließlich aller `Card`-Objekte) zurück an das Frontend.

6. **Alle Decks anzeigen**:
    - Wenn ein Benutzer alle Decks anzeigen möchte, senden Sie eine Anfrage vom Frontend an Rust.
    - Rust sendet die Informationen aller `Deck`-Objekte zurück an das Frontend.

Bitte beachten Sie, dass dies eine allgemeine Anleitung ist und je nach den spezifischen Anforderungen Ihrer Anwendung angepasst werden kann. Es ist auch wichtig, Fehlerbehandlungen und Validierungen hinzuzufügen, um die Robustheit Ihrer Anwendung zu gewährleisten. Viel Erfolg bei Ihrem Projekt! 😊

Es gibt verschiedene Möglichkeiten, wie Sie die Decks in Rust speichern können. Hier sind einige Optionen:

1. **In-Memory-Speicherung**: Sie können eine Datenstruktur wie `Vec<Deck>` oder `HashMap<String, Deck>` verwenden, um die Decks im Speicher zu speichern. Dies ist einfach zu implementieren, aber die Daten gehen verloren, wenn das Programm beendet wird.

2. **Dateisystem**: Sie können die Decks als Dateien auf dem Dateisystem speichern. Jedes Deck könnte seine eigene Datei sein, und die Karten könnten als Zeilen in dieser Datei gespeichert werden. Sie könnten die Daten im JSON-Format speichern, da Ihre Strukturen bereits das `serde::Serialize`-Trait implementieren.

3. **Datenbank**: Für eine robustere Lösung könnten Sie eine Datenbank verwenden, um die Decks zu speichern. Es gibt verschiedene Datenbanken, die Sie verwenden könnten, z.B. SQLite, PostgreSQL, etc. Es gibt Rust-Bibliotheken, die die Interaktion mit diesen Datenbanken erleichtern.

Hier ist ein einfaches Beispiel, wie Sie die Decks im Speicher mit einem `Vec<Deck>` speichern könnten:

```rust
#[derive(serde::Serialize)]
pub struct Card {
    question: String,
    answer: String,
}

#[derive(serde::Serialize)]
pub struct Deck {
    name: String,
    cards: Vec<Card>,
}

pub struct App {
    decks: Vec<Deck>,
}

impl App {
    pub fn new() -> Self {
        Self { decks: Vec::new() }
    }

    pub fn add_deck(&mut self, deck: Deck) {
        self.decks.push(deck);
    }

    // Weitere Methoden zum Hinzufügen/Bearbeiten/Löschen von Karten...
}
```
In diesem Beispiel würde die `App`-Struktur alle Decks speichern.


use rusqlite::{params, Connection, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct Card {
    question: String,
    answer: String,
}

#[derive(Serialize)]
pub struct Deck {
    name: String,
    cards: Vec<Card>,
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
                question TEXT,
                answer TEXT,
                deck_name TEXT,
                FOREIGN KEY(deck_name) REFERENCES decks(name)
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn add_deck(&self, deck: &Deck) -> Result<()> {
        self.conn.execute(
            "INSERT INTO decks (name) VALUES (?1)",
            params![deck.name],
        )?;

        for card in &deck.cards {
            self.conn.execute(
                "INSERT INTO cards (question, answer, deck_name) VALUES (?1, ?2, ?3)",
                params![card.question, card.answer, deck.name],
            )?;
        }

        Ok(())
    }

    // Weitere Methoden zum Hinzufügen/Bearbeiten/Löschen von Karten...
}

*/