use repository::Repository;
use vocab_finder::{ConcurrentVocabBuilder, VocabBuilder};

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn create_list(name: &str, content: String, state: tauri::State<ConcurrentVocabBuilder>) -> String {
    state.start(content);
    println!("Hello!");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn request_progress(state: tauri::State<ConcurrentVocabBuilder>) -> f64 {
    println!("Progress!");
    let x = state.progress();
    println!("Progress! {x}");
    x
}

fn main() {
    tauri::Builder::default()
        .manage(ConcurrentVocabBuilder::default())
        .invoke_handler(tauri::generate_handler![create_list, request_progress])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct StateManager {
    repo: Repository,
    pub vocab_builder: Box<dyn VocabBuilder>,
}

impl StateManager {
    fn new(db_path: &str, vocab_builder: Box<dyn VocabBuilder>) -> Self {
        StateManager {
            repo: Repository::new(db_path).expect("Error while trying to create/use database."),
            vocab_builder,
        }
    }

    fn write_new_list(&self) {}
}
