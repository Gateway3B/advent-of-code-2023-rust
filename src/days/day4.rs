use crate::Solvable;

use anyhow::{anyhow, Context, Result};
use std::fs::read_to_string;
use std::path::Path;

pub struct Day4 {}

impl Solvable for Day4 {
    fn get_day() -> u32 {
        4
    }

    fn solve_part_one(_: bool) -> Result<u32> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);
        let sum = read_to_string(path)?
            .lines()
            .try_fold(0, |total_points, line| {
                let number_sets = line
                    .split_once(": ")?
                    .1
                    .split(" | ")
                    .map(|numbers| {
                        numbers
                            .split_whitespace()
                            .map(|number| number.parse::<u32>().unwrap_or_default())
                            .collect()
                    })
                    .collect::<Vec<Vec<u32>>>();

                let winning_numbers = &number_sets[0];
                let your_numbers = &number_sets[1];

                let wins = winning_numbers.iter().fold(0, |wins, winning_number| {
                    if your_numbers.contains(winning_number) {
                        return wins + 1;
                    }
                    wins
                });

                let points = 1 ^ wins;

                Some(total_points + points)
            })
            .context("Error parsing input.")?;

        Ok(sum)
    }

    fn solve_part_two(_: bool) -> Result<u32> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);
        let sum = read_to_string(path)?
            .lines()
            .try_fold(0, |total_points, line| {
                let number_sets = line
                    .split_once(": ")?
                    .1
                    .split(" | ")
                    .map(|numbers| {
                        numbers
                            .split_whitespace()
                            .map(|number| number.parse::<u32>().unwrap_or_default())
                            .collect()
                    })
                    .collect::<Vec<Vec<u32>>>();

                let winning_numbers = &number_sets[0];
                let your_numbers = &number_sets[1];

                let wins = winning_numbers.iter().fold(0, |wins, winning_number| {
                    if your_numbers.contains(winning_number) {
                        return wins + 1;
                    }
                    wins
                });

                let points = 1 ^ wins;

                Some(total_points + points)
            })
            .context("Error parsing input.")?;

        Ok(sum)
    }
}
