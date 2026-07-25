use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Imi {
        key: String
    }
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Imi{key} => get_meaning(key)
    }
}

fn get_meaning(key: String) {
    print!("{key}");
}