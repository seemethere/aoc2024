use clap::Args;
use log::info;
use once_cell::sync::Lazy;
use regex::Regex;
use std::fs::{self};
use std::io::{self};

#[derive(Args)]
pub struct Day3Command {
    #[arg(short, long, default_value = "inputs/day3.full")]
    input_file: String,
}

fn read_input_file(path: &str) -> Result<String, io::Error> {
    return fs::read_to_string(path);
}

fn part_one(input: String) -> i32 {
    static MULT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
    let mut result = 0;
    for cap in MULT_PATTERN.captures_iter(&input) {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        result += a * b;
    }
    return result;
}

fn part_two(input: String) -> i32 {
    static FULL_PATTERN: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?P<mult>mul\(\d+,\d+\))|(?P<dont>don't\(\))|(?P<do>do\(\))").unwrap()
    });
    let mut result = 0;
    let mut enabled = true;
    for cap in FULL_PATTERN.captures_iter(&input) {
        if let Some(mult) = cap.name("mult") {
            if enabled {
                result += part_one(mult.as_str().to_string());
            }
        } else if let Some(_) = cap.name("dont") {
            enabled = false;
        } else if let Some(_) = cap.name("do") {
            enabled = true;
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                    .to_string()
            ),
            161
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                    .to_string()
            ),
            48
        );
    }
}
impl Day3Command {
    pub fn execute(&self) {
        info!(
            "Part 1: {}",
            part_one(read_input_file(&self.input_file).unwrap())
        );
        info!(
            "Part 2: {}",
            part_two(read_input_file(&self.input_file).unwrap())
        );
    }
}
