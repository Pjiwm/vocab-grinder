use std::sync::Arc;

use repository::models::Word;
use repository::Repository;
use tauri::State;
use vocab_finder::{ConcurrentVocabBuilder, VocabBuilder};

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn create_list(content: String, state: State<StateManager>) -> Result<String, String> {
    state.create_new_list(content)
}

#[tauri::command]
fn request_progress(state: State<StateManager>) -> f64 {
    state.vocab_builder.progress()
}

#[tauri::command]
fn compute_list(list_name: &str, state: State<StateManager>) -> Result<i64, String> {
    state.compute_new_list(list_name)
}

#[tauri::command]
fn is_computing_done(state: State<StateManager>) -> bool {
    state.vocab_builder.results_done()
}

#[tauri::command]
fn save_list(list_id: i32, state: State<StateManager>) -> Result<(), String> {
    state.save_list(list_id)
}

fn main() {
    tauri::Builder::default()
        .manage(StateManager::new(
            "../list_db.db",
            Arc::new(ConcurrentVocabBuilder::default()),
        ))
        .invoke_handler(tauri::generate_handler![
            create_list,
            request_progress,
            compute_list,
            is_computing_done,
            save_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct StateManager {
    repo: Repository,
    pub vocab_builder: Arc<dyn VocabBuilder + Send + Sync>,
}

impl StateManager {
    fn new(db_path: &str, vocab_builder: Arc<dyn VocabBuilder + Send + Sync>) -> Self {
        StateManager {
            repo: Repository::new(db_path).expect("Error while trying to create/use database."),
            vocab_builder,
        }
    }

    fn create_new_list(&self, content: String) -> Result<String, String> {
        if self.vocab_builder.is_on() {
            Err("Already creating a list.".to_string())
        } else {
            self.vocab_builder.start(content);
            Ok("Creating new list".to_string())
        }
    }

    fn compute_new_list(&self, list_name: &str) -> Result<i64, String> {
        if !self.vocab_builder.is_on() {
            Err("Text is not being analyzed right now.".to_string())
        } else if self.vocab_builder.progress() != 100f64 {
            Err("Text is not done being analyzed.".to_string())
        } else {
            self.vocab_builder.compute_result();
            self.repo
                .create_list(list_name)
                .map_err(|_| "Could not create list in database.".to_string())
        }
    }

    fn save_list(&self, list_id: i32) -> Result<(), String> {
        if !self.vocab_builder.results_done() {
            Err("Text was not done computing, failed to store to database.".to_string())
        } else {
            let words = self.vocab_builder.get_result();
            let repo = self.repo.clone();
            std::thread::spawn(move || {
                println!("WRITING TO DBBB!");
                for vocab_word in words {
                    let _ = repo.add_word_to_list(
                        list_id,
                        &Word {
                            word: vocab_word.0.kanji,
                            reading: vocab_word.0.reading,
                            list_id,
                            id: None,
                            frequency: vocab_word.1 as i32,
                            translation: vocab_word.0.meanings.join(", "),
                        },
                    );
                }
            });
            Ok(())
        }
    }
}
