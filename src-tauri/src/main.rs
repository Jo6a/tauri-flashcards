// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[derive(serde::Serialize)]
struct Deck {
    name: String,
}

#[tauri::command]
fn get_text() -> String {
    "Dies ist ein Beispieltext aus Rust.\n
    Dies ist ein Beispieltext aus Rust.\n
    Dies ist ein Beispieltext aus Rust.\n
    Dies ist ein Beispieltext aus Rust.\n
    Dies ist ein Beispieltext aus Rust.\n
    Dies ist ein Beispieltext aus Rust.\n
    Dies ist ein Beispieltext aus Rust.\n
    Dies ist ein Beispieltext aus Rust.\n
    Dies ist ein Beispieltext aus Rust.\n
    Dies ist ein Beispieltext aus Rust.\n".to_string()
}

#[tauri::command]
fn get_deck_names() -> Vec<Deck> {
    let mut decks = Vec::new();

    // Fügen Sie hier Code hinzu, um die tatsächlichen Decknamen abzurufen
    // Im Moment fügen wir einfach einige Beispiele hinzu
    decks.push(Deck { name: "Deck 1".to_string() });
    decks.push(Deck { name: "Deck 2".to_string() });
    decks.push(Deck { name: "Deck 3".to_string() });

    decks
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_text, get_deck_names])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
