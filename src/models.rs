use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SearchRequest {
    pub query: String,
}

#[derive(Debug, Deserialize)]
pub struct DictionaryResponse {
    pub words: Vec<WordEntry>,
}

#[derive(Debug, Deserialize)]
pub struct WordEntry {
    pub reading: Reading,
    pub senses: Vec<Sense>,
}

#[derive(Debug, Deserialize)]
pub struct Sense {
    pub glosses: Vec<String>,
    pub language: String,
}

#[derive(Debug, Deserialize)]
pub struct Reading {
    pub kana: String,
    pub kanji: String,
    pub furigana: String,
}
