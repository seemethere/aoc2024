use std::fs::{self};
use std::io::{self, BufRead};

use clap::Args;
use log::info;

#[derive(Args)]
pub struct Day2Command {
    #[arg(short, long, default_value = "inputs/day2.full")]
    input_file: String,
}

fn read_input_file(path: &str) -> Result<Vec<Vec<i32>>, io::Error> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut list: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        let numbers: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        list.push(numbers);
    }
    return Ok(list);
}

fn step_is_safe(ascending: bool, step: i32, prev: i32) -> bool {
    let diff = step - prev;
    // originally ascending and now descending or vice versa
    if ascending && diff <= 0 || !ascending && diff >= 0 {
        return false;
    }

    // difference is greater than 3
    if diff.abs() > 3 {
        return false;
    }
    return true;
}

fn vector_is_safe(vec: &Vec<i32>, dampener: i32) -> bool {
    let mut i = 0;
    let ascending = (vec[i + 1] - vec[i]) > 0;
    while i < vec.len() - 1 {
        if !step_is_safe(ascending, vec[i + 1], vec[i]) {
            if dampener == 0 {
                return false;
            }
            return vector_is_safe(&[&vec[..i], &vec[i + 1..]].concat(), dampener - 1);
        }
        i += 1;
    }
    return true;
}

fn count_safe(list: &Vec<Vec<i32>>, dampener: i32) -> i32 {
    let mut safe_count = 0;
    for vec in list {
        if vector_is_safe(vec, dampener) {
            safe_count += 1;
        }
    }
    return safe_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {
        assert_eq!(vector_is_safe(&vec![7, 6, 4, 2, 1], 0), true);
        assert_eq!(vector_is_safe(&vec![1, 2, 7, 8, 9], 0), false);
        assert_eq!(vector_is_safe(&vec![9, 7, 6, 2, 1], 0), false);
        assert_eq!(vector_is_safe(&vec![1, 3, 2, 4, 5], 0), false);
        assert_eq!(vector_is_safe(&vec![8, 6, 4, 4, 1], 0), false);
        assert_eq!(vector_is_safe(&vec![1, 3, 6, 7, 9], 0), true);
    }

    #[test]
    fn test_is_safe_with_dampener() {
        assert_eq!(vector_is_safe(&vec![7, 6, 4, 2, 1], 1), true);
        assert_eq!(vector_is_safe(&vec![1, 2, 7, 8, 9], 1), false);
        assert_eq!(vector_is_safe(&vec![9, 7, 6, 2, 1], 1), false);
        assert_eq!(vector_is_safe(&vec![1, 3, 2, 4, 5], 1), true);
        assert_eq!(vector_is_safe(&vec![8, 6, 4, 4, 1], 1), true);
        assert_eq!(vector_is_safe(&vec![1, 3, 6, 7, 9], 1), true);
    }

    #[test]
    fn test_count_safe() {
        let list = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(count_safe(&list, 0), 2);
    }

    #[test]
    fn test_count_safe_with_dampener() {
        let list = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(count_safe(&list, 1), 4);
    }
}

impl Day2Command {
    pub fn execute(&self) {
        let list1 = read_input_file(&self.input_file).ok().unwrap();
        let safe_count = count_safe(&list1, 0);
        // NOTE: This doesn't work on the full input, it functions fine on the small input though. Might need to go back and debug why
        let safe_count_with_dampener = count_safe(&list1, 1);
        info!("Safe count (Dampener: {}): {}", 0, safe_count);
        info!("Safe count (Dampener: {}): {}", 1, safe_count_with_dampener);
    }
}
