mod day1;

pub use day1::Day1Command;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Days {
    Day1(Day1Command),
}

impl Days {
    pub fn execute(&self) {
        match self {
            Days::Day1(cmd) => cmd.execute(),
        }
    }
}
