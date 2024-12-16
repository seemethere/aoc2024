mod days;

use crate::days::Days;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Days>,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    cli.command.unwrap().execute();
}
