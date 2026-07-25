mod models;
mod api;
mod db;

use clap::{Parser, Subcommand};

use crate::{api::get_meaning, models::Sense};
use crate::db::db::Database;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Look up the meaning of a Japanese word
    Imi { 
        key: String,
        #[arg(short, long)]
        verbose: bool
    },
    Add {
        japanese: String,
    },
    List
}

#[tokio::main]
async fn main() {

    let args = Args::parse();
    let db = match Database::init_db("sqlite://jp.db?mode=rwc").await {
        Ok(db) => db,
        Err(error) => {
            eprint!("Failed to connect to database: {error}");
            return;
        }
    };

    match args.cmd {
        Commands::Imi { key, verbose } => imi_handler(&key, verbose).await,
        Commands::Add { japanese } => add_handler(&japanese, &db).await,
        Commands::List => list_handler(&db).await,
    }
}

async fn imi_handler(key: &str, verbose: bool) {
    println!("{key}:\n");

    match (get_meaning(key).await, verbose) {
        (Ok(response), true) => {
            for word in response.words {
                println!("-----------------------\n");
                println!("Kanji:\n {}\n", word.reading.kanji);
                println!("Reading:\n {}\n", word.reading.kana);
                for sense in word.senses {
                    println!("Meaning:");
                    for gloss in sense.glosses {
                        println!("{gloss}")
                    }
                    println!("");
                }
            }
        },
        (Ok(response), false) => {
            for word in response.words {
                println!("-----------------------\n");
                for sense in word.senses {
                    for gloss in sense.glosses {
                        println!("{gloss}")
                    }
                    println!("");
                }
            }
        }
        (Err(error), _) => eprint!("Error: {error}"),
    }
}

async fn add_handler(japanese: &str, db: &Database) {
    let response = match get_meaning(japanese).await {
        Ok(response) => response,
        Err(error) => {
            eprint!("Error: {error}");
            return;
        }
    };

    let meaning = &response.words[0].senses[0].glosses[0];

    if let Err(error) = db.add_word(japanese, meaning).await {
        eprint!("Error: {error}");
    }
}

async fn list_handler(db: &Database) {
    match db.list_words().await {
        Ok(words) => {
            for word in words {
                println!("{}: {}", word.japanese, word.english);
            }
        },
        Err(error) => eprint!("Error: {error}")
    }
}
