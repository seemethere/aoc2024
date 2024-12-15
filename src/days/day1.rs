use clap::Args;

use itertools::{interleave, Itertools};
use log::{debug, info};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Args)]
pub struct Day1Command {
    #[arg(short, long, default_value = "inputs/day1.full")]
    input_file: String,
}

fn read_input_file(path: &str) -> Result<(Vec<i32>, Vec<i32>), io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let numbers: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if numbers.len() != 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid data"));
        }
        list1.push(numbers[0]);
        list2.push(numbers[1]);
    }
    return Ok((list1, list2));
}

fn sort_vector_with_positions(input_vec: &Vec<i32>) -> Vec<i32> {
    return input_vec.iter().cloned().sorted().collect();
}

impl Day1Command {
    pub fn execute(&self) {
        let inputs = read_input_file(&self.input_file).ok();
        inputs.map(|(list1, list2)| {
            let sorted_list1 = sort_vector_with_positions(&list1);
            let sorted_list2 = sort_vector_with_positions(&list2);
            let mut total_distance = 0;
            let mut i = 0;
            while i < sorted_list1.len() {
                total_distance += (sorted_list1[i] - sorted_list2[i]).abs();
                i += 1;
            }
            info!("Total distance: {}", total_distance);
        });
    }
}
