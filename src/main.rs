mod api;
mod db;
mod models;
mod tui;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::io;

use crate::api::get_meaning;
use crate::db::Database;
use crate::tui::App;

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
        verbose: bool,
    },
    Add {
        japanese: String,
    },
    List,
    Remove {
        japanese: String,
    },
    Tui
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Imi { key, verbose } => imi_handler(&key, verbose).await,
        Commands::Add { japanese } => add_handler(&japanese).await,
        Commands::List => list_handler().await,
        Commands::Remove { japanese } => remove_handler(&japanese).await,
        Commands::Tui => tui_handler().expect("REASON"),
    }
}

async fn init_db() -> Option<Database> {
    let Some(home_dir) = home_dir() else {
        eprintln!("Could not determine the user's home directory");
        return None;
    };

    let db_dir = home_dir.join(".jp-cli");
    if let Err(error) = std::fs::create_dir_all(&db_dir) {
        eprintln!("Failed to create {}: {error}", db_dir.display());
        return None;
    }

    let db_path = db_dir.join("jp.db");
    match Database::init_db(&db_path).await {
        Ok(db) => Some(db),
        Err(error) => {
            eprintln!(
                "Failed to connect to database at {}: {error}",
                db_path.display()
            );
            None
        }
    }
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
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
                    println!();
                }
            }
        }
        (Ok(response), false) => {
            for word in response.words {
                println!("-----------------------\n");
                for sense in word.senses {
                    for gloss in sense.glosses {
                        println!("{gloss}")
                    }
                    println!();
                }
            }
        }
        (Err(error), _) => eprint!("Error: {error}"),
    }
}

async fn add_handler(japanese: &str) {
    let Some(db) = init_db().await else {
        return;
    };

    let response = match get_meaning(japanese).await {
        Ok(response) => response,
        Err(error) => {
            eprint!("Error: {error}");
            return;
        }
    };

    let Some(meaning) = response
        .words
        .first()
        .and_then(|word| word.senses.first())
        .and_then(|sense| sense.glosses.first())
    else {
        eprintln!("No meaning found for '{japanese}'");
        return;
    };

    if let Err(error) = db.add_word(japanese, meaning).await {
        eprintln!("Error: {error}");
    }
}

async fn list_handler() {
    let Some(db) = init_db().await else {
        return;
    };

    match db.list_words().await {
        Ok(words) => {
            for word in words {
                println!("{}: {}", word.japanese, word.english);
            }
        }
        Err(error) => eprint!("Error: {error}"),
    }
}

async fn remove_handler(word: &str) {
    let Some(db) = init_db().await else {
        return;
    };

    if let Err(error) = db.remove_word(word).await {
        eprint!("Error: {error}")
    }
}

fn tui_handler() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}
