use clap::Args;

use itertools::Itertools;
use log::info;
use std::collections::HashMap;
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

fn sort_vector(input_vec: &Vec<i32>) -> Vec<i32> {
    return input_vec.iter().cloned().sorted().collect();
}

fn compute_total_distance(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let sorted_list1 = sort_vector(list1);
    let sorted_list2 = sort_vector(list2);
    let mut total_distance = 0;
    let mut i = 0;
    while i < sorted_list1.len() {
        total_distance += (sorted_list1[i] - sorted_list2[i]).abs();
        i += 1;
    }
    return total_distance;
}

fn compute_similarity_score(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut occurence_map: HashMap<i32, i32> = HashMap::new();
    let mut similarity_score = 0;

    for x in list2.iter() {
        if occurence_map.contains_key(&x) {
            occurence_map.insert(*x, occurence_map[&x] + 1);
        } else {
            occurence_map.insert(*x, 1);
        }
    }
    for key in list1 {
        if occurence_map.contains_key(&key) {
            similarity_score += key * occurence_map[&key];
        }
    }
    return similarity_score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_vector_with_positions() {
        let input_vec = vec![3, 1, 2];
        let expected_output = vec![1, 2, 3];
        assert_eq!(sort_vector(&input_vec), expected_output);
    }

    #[test]
    fn test_compute_total_distance() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(compute_total_distance(&list1, &list2), 11);
    }

    #[test]
    fn test_compute_similarity_score() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(compute_similarity_score(&list1, &list2), 31);
    }
}

impl Day1Command {
    pub fn execute(&self) {
        let (list1, list2) = read_input_file(&self.input_file).ok().unwrap();
        let total_distance = compute_total_distance(&list1, &list2);
        let similarity_score = compute_similarity_score(&list1, &list2);
        info!("Total distance: {}", total_distance);
        info!("Similarity score: {}", similarity_score);
    }
}
