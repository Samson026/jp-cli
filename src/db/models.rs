use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Word {
    pub id: i64,
    pub japanese: String,
    pub english: String,
}
