mod day1;
mod day2;
mod day3;
mod day4;

pub use day1::Day1Command;
pub use day2::Day2Command;
pub use day3::Day3Command;
pub use day4::Day4Command;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Days {
    Day1(Day1Command),
    Day2(Day2Command),
    Day3(Day3Command),
    Day4(Day4Command),
}

impl Days {
    pub fn execute(&self) {
        match self {
            Days::Day1(cmd) => cmd.execute(),
            Days::Day2(cmd) => cmd.execute(),
            Days::Day3(cmd) => cmd.execute(),
            Days::Day4(cmd) => cmd.execute(),
        }
    }
}
