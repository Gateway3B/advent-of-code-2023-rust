use crate::Solvable;

use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;

pub struct Day1 {}

impl Solvable for Day1 {
    fn get_day() -> u32 {
        1
    }

    fn solve_part_one() -> Result<u32> {
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
            .context("")?;
    
        Ok(sum)
    }
    
    fn solve_part_two() -> Result<u32> {
        let path = Path::new("src/inputs/day1.txt");
        let sum = read_to_string(path)?
            .lines()
            .try_fold(0, |sum, line| {
                let number_strs = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

                let min_number_str = number_strs
                    .iter()
                    .enumerate()
                    .fold(None::<(usize, usize)>, |min, (number, number_str)| {
                        let mut new_min = min;
                        let res_index = line.find(number_str);
                        if let Some(res_index) = res_index {
                            if let Some((_, min_index)) = min {
                                if res_index < min_index {
                                    new_min = Some((number + 1, res_index));
                                }
                            } else {
                                new_min = Some((number + 1, res_index));
                            }
                        }
                        new_min
                    });
                
                let max_number_str = number_strs
                    .iter()
                    .enumerate()
                    .fold(None::<(usize, usize)>, |max, (number, number_str)| {
                        let mut new_max = max;
                        let res_index = line.find(number_str);
                        if let Some(res_index) = res_index {
                            if let Some((_, max_index)) = max {
                                if res_index > max_index {
                                    new_max = Some((number + 1, res_index));
                                }
                            } else {
                                new_max = Some((number + 1, res_index));
                            }
                        }
                        new_max
                    });
                
                let first_index = line.chars().position(|char| char.is_numeric())?;
                let last_index = line.chars().rposition(|char| char.is_numeric())?;
                
    
                let number = format!("{first}{last}");
                let number: u32 = number.parse().ok()?;
                
                Some(sum + number)
            })
            .context("")?;
    
        Ok(sum)
    }
}
