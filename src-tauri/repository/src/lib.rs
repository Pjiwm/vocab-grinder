pub mod models;
mod schema;
use crate::models::{List, Word};
use crate::schema::{CREATE_LIST_TABLE, CREATE_WORD_TABLE};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Result};
use std::sync::Arc;
#[derive(Clone)]
pub struct Repository {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl Repository {
    pub fn new(db_path: &str) -> Result<Self> {
        let manager = SqliteConnectionManager::file(db_path);
        let pool = Pool::builder()
            .build(manager)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let conn = pool
            .get()
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        // Initialize the database schema
        conn.execute(CREATE_LIST_TABLE, [])?;
        conn.execute(CREATE_WORD_TABLE, [])?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    fn get_connection(&self) -> Result<r2d2::PooledConnection<SqliteConnectionManager>> {
        self.pool
            .get()
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))
    }

    pub fn create_list(&self, name: &str) -> Result<i64> {
        let conn = self.get_connection()?;
        conn.execute("INSERT INTO list (name) VALUES (?1)", params![name])?;
        let list_id = conn.last_insert_rowid();
        Ok(list_id)
    }

    pub fn add_word_to_list(&self, list_id: i32, word: &Word) -> Result<()> {
        let conn = self.get_connection()?;
        conn.execute(
            "INSERT INTO word (list_id, word, reading, translation, frequency) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![list_id, word.word, word.reading, word.translation, word.frequency],
        )?;
        Ok(())
    }

    pub fn get_lists(&self) -> Result<Vec<List>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT id, name FROM list")?;
        let list_iter = stmt.query_map([], |row| {
            Ok(List {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let mut lists = Vec::new();
        for list in list_iter {
            lists.push(list?);
        }
        Ok(lists)
    }

    pub fn get_words_for_list(&self, list_id: i32) -> Result<Vec<Word>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT id, list_id, word, reading, translation, frequency FROM word WHERE list_id = ?1")?;
        let word_iter = stmt.query_map([list_id], |row| {
            Ok(Word {
                id: row.get(0)?,
                list_id: row.get(1)?,
                word: row.get(2)?,
                reading: row.get(3)?,
                translation: row.get(4)?,
                frequency: row.get(5)?,
            })
        })?;

        let mut words = Vec::new();
        for word in word_iter {
            words.push(word?);
        }
        Ok(words)
    }
}
