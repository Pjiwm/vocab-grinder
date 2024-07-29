use std::sync::Mutex;

use vocab_finder::{ConcurrentVocabBuilder, VocabBuilder};
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn create_list(
    name: &str,
    content: String,
    state: tauri::State<Mutex<ConcurrentVocabBuilder>>,
) -> String {
    state.lock().unwrap().start(content);
    println!("Hello!");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn request_progress(state: tauri::State<Mutex<ConcurrentVocabBuilder>>) -> f64 {
    println!("Progress!");
    state.lock().unwrap().progress()
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(ConcurrentVocabBuilder::default()))
        .invoke_handler(tauri::generate_handler![create_list, request_progress])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
