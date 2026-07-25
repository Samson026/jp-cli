use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Word {
    pub japanese: String,
    pub english: String,
}
