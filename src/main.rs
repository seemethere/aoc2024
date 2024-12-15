mod days;

use crate::days::Days;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Days>,
}

#[derive(Subcommand)]
enum Commands {
    Day1 {
        #[arg(required = true)]
        input_file: String,
    },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    cli.command.unwrap().execute();
}
