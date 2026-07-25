use reqwest;
use crate::models::SearchRequest;
use crate::models::DictionaryResponse;

pub async fn get_meaning(word: &str) -> Result<DictionaryResponse, reqwest::Error> {
    let client = reqwest::Client::new();

    let body = SearchRequest{
        query: word.to_string()
    };

    let reponse: DictionaryResponse = client
        .post("https://jotoba.de/api/search/words")
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    Ok(reponse)
}