use crate::Solvable;

use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;

pub struct Day1 {}

impl Solvable for Day1 {
    fn get_day() -> u32 {
        1
    }

    fn solve_part_one(_: bool) -> Result<i64> {
        let path = Path::new("src/inputs/day1.txt");
        let sum = read_to_string(path)?
            .lines()
            .try_fold(0, |sum, line| {
                let first = line.chars().find(|char| char.is_numeric())?;
                let last = line.chars().rev().find(|char| char.is_numeric())?;

                let number = format!("{first}{last}");
                let number: u32 = number.parse().ok()?;

                Some(sum + number)
            })
            .context("Could not find numbers.")?;

        let sum = i64::try_from(sum)?;
        Ok(sum)
    }

    fn solve_part_two(debug: bool) -> Result<i64> {
        let path = Path::new("src/inputs/day1.txt");
        let sum = read_to_string(path)?
            .lines()
            .try_fold(0, |sum, line| {
                let number_strs = [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ];

                let min_number_str = number_strs.iter().enumerate().fold(
                    None::<(usize, char)>,
                    |min, (index, number_str)| {
                        let mut new_min = min;
                        let number = u32::try_from(index + 1).ok()?;
                        let number = char::from_digit(number, 10)?;

                        let res_index = line.find(number_str);
                        if let Some(res_index) = res_index {
                            if let Some((min_index, _)) = min {
                                if res_index < min_index {
                                    new_min = Some((res_index, number));
                                }
                            } else {
                                new_min = Some((res_index, number));
                            }
                        }
                        new_min
                    },
                );

                let max_number_str = number_strs.iter().enumerate().fold(
                    None::<(usize, char)>,
                    |max, (index, number_str)| {
                        let mut new_max = max;
                        let number = u32::try_from(index + 1).ok()?;
                        let number = char::from_digit(number, 10)?;

                        let res_index = line.rfind(number_str);
                        if let Some(res_index) = res_index {
                            if let Some((max_index, _)) = max {
                                if res_index > max_index {
                                    new_max = Some((res_index, number));
                                }
                            } else {
                                new_max = Some((res_index, number));
                            }
                        }
                        new_max
                    },
                );

                let min_number = line
                    .chars()
                    .enumerate()
                    .find_map(|(index, char)| char.is_numeric().then(|| (index, char)));
                let max_number = line.chars().rev().enumerate().find_map(|(index, char)| {
                    char.is_numeric().then(|| (line.len() - index, char))
                });

                let first = match (min_number, min_number_str) {
                    (Some(min_number), Some(min_number_str)) => {
                        if min_number.0 < min_number_str.0 {
                            min_number.1
                        } else {
                            min_number_str.1
                        }
                    }
                    (Some(min_number), None) => min_number.1,
                    (None, Some(min_number_str)) => min_number_str.1,
                    (None, None) => return None,
                };

                let last = match (max_number, max_number_str) {
                    (Some(max_number), Some(max_number_str)) => {
                        if max_number.0 > max_number_str.0 {
                            max_number.1
                        } else {
                            max_number_str.1
                        }
                    }
                    (Some(max_number), None) => max_number.1,
                    (None, Some(max_number_str)) => max_number_str.1,
                    (None, None) => return None,
                };

                let number = format!("{first}{last}");
                let number: u32 = number.parse().ok()?;

                if debug {
                    println!("Line: {:?}", line);

                    println!("Min String: {:?}", min_number_str);
                    println!("Max String: {:?}", max_number_str);

                    println!("Min Number: {:?}", min_number);
                    println!("Max Number: {:?}", max_number);

                    println!("Number: {:?}{:?}", first, last);
                    println!("");
                }

                Some(sum + number)
            })
            .context("")?;

        let sum = i64::try_from(sum)?;
        Ok(sum)
    }
}
