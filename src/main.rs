mod models;
mod api;

use clap::{Parser, Subcommand};

use crate::{api::get_meaning};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Imi { key: String },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Imi { key } => print_meaning(key).await,
    }
}

async fn print_meaning(key: String) {
    println!("{key}:\n");

    match get_meaning(key).await {
        Ok(response) => {
            for word in response.words {
                println!("Reading:\n {}\n", word.reading.kana);
                for sense in word.senses {
                    print!("Meaing:\n");
                    for gloss in sense.glosses {
                        println!("{gloss}")
                    }
                }
            }
        },
        Err(error) => eprint!("Error: {error}"),
    }
}
