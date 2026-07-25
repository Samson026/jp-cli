

use sqlx::sqlite::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Row;
use super::models::Word;

pub struct Database {
    pool: SqlitePool
}

impl Database {
    pub async fn init_db(db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS words (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                japanese TEXT NOT NULL UNIQUE,
                english TEXT NOT NULL
            )"
        )
            .execute(&pool)
            .await?;

        Ok(Self { pool })
    }

    pub async fn add_word(&self, japanese: &str, english: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO words (japanese, english)
            VALUES (?, ?)", 
        )
        .bind(japanese)
        .bind(english)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_words(&self) -> Result<Vec<Word>, sqlx::Error> {
        sqlx::query_as::<_, Word>(
            "SELECT * FROM words"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn remove_word(&self, word: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM words WHERE japanese == ?"
        )
        .bind(word)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}