mod day1;
mod day2;

pub use day1::Day1Command;
pub use day2::Day2Command;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Days {
    Day1(Day1Command),
    Day2(Day2Command),
}

impl Days {
    pub fn execute(&self) {
        match self {
            Days::Day1(cmd) => cmd.execute(),
            Days::Day2(cmd) => cmd.execute(),
        }
    }
}
