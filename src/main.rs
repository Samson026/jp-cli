mod api;
mod cli;
mod db;
mod tui;

use clap::{Parser, Subcommand};

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
    Tui,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Imi { key, verbose } => cli::imi_handler(&key, verbose).await,
        Commands::Add { japanese } => cli::add_handler(&japanese).await,
        Commands::List => cli::list_handler().await,
        Commands::Remove { japanese } => cli::remove_handler(&japanese).await,
        Commands::Tui => cli::tui_handler().await.expect("REASON"),
    }
}
