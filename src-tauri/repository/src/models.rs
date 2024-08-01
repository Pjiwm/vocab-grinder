use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    pub id: Option<i32>,
    pub list_id: i32,
    pub word: String,
    pub reading: String,
    pub translation: String,
    pub frequency: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub id: Option<i32>,
    pub name: String,
}
