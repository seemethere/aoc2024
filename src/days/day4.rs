use std::fs::{self};
use std::io::{self, BufRead};

use clap::Args;
use log::info;

#[derive(Args)]
pub struct Day4Command {
    #[arg(short, long, default_value = "inputs/day4.full")]
    input_file: String,
}

fn read_input_file(path: &str) -> Result<Vec<Vec<char>>, io::Error> {
    // iterate through the file and collect each line as a vector of characters
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut list: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let chars: Vec<char> = line.unwrap().chars().collect();
        list.push(chars);
    }
    return Ok(list);
}

fn part_one(input: Vec<Vec<char>>, word: String) -> i32 {
    let moves: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let word_vec: Vec<char> = word.chars().collect();
    let mut num_found: i32 = 0;
    for x in 0..input.len() {
        for y in 0..input.len() {
            if input[x][y] == word_vec[0] {
                for mv in moves.iter() {
                    let found: bool = 'done: {
                        for w in 0..word_vec.len() - 1 {
                            let inner_x = (x as i32) + mv.0 + ((w as i32) * mv.0);
                            let inner_y = (y as i32) + mv.1 + ((w as i32) * mv.1);
                            // Check if out of bounds and if the next character is not the next character in the word
                            if inner_x as usize > input.len() - 1
                                || inner_y as usize > input.len() - 1
                                || inner_x < 0
                                || inner_x < 0
                                || input[inner_x as usize][inner_y as usize] != word_vec[w + 1]
                            {
                                break 'done false;
                            }
                        }
                        true
                    };
                    if found {
                        num_found += 1;
                    }
                }
            }
        }
    }
    return num_found;
}

// TODO: Implement part two

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    ";

        // Convert input into a vector of vectors of characters
        let grid: Vec<Vec<char>> = input
            .lines() // Split into lines
            .filter(|line| !line.trim().is_empty()) // Ignore empty lines
            .map(|line| line.trim().chars().collect()) // Trim and collect chars
            .collect();
        assert_eq!(part_one(grid, String::from("XMAS")), 18);
    }
}

impl Day4Command {
    pub fn execute(&self) {
        info!(
            "Part 1: {}",
            part_one(
                read_input_file(&self.input_file).unwrap(),
                String::from("XMAS")
            )
        );
    }
}
