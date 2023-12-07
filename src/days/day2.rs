use crate::Solvable;

use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;

pub struct Day2 {}

impl Solvable for Day2 {
    fn get_day() -> u32 {
        2
    }
    
    fn solve_part_one() -> Result<u32> {
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;
    
        let path = Path::new("src/inputs/day2.txt");
        let sum = read_to_string(path)?
            .lines()
            .enumerate()
            .try_fold(0, |sum, (index, line)| {
                let possible = line
                    .strip_prefix(&format!("Game {}: ", index + 1))?
                    .split_terminator("; ")
                    .all(|hand| {
                        hand.split_terminator(", ").all(|color_cube| {
                            let mut iter = color_cube.split_terminator(" ");
                            let number = if let Some(number) = iter.next() { number } else { return false; };
                            let number: u32 = if let Ok(number) = number.parse() { number } else { return false; };
                            let color = if let Some(color) = iter.next() { color} else { return false; };
    
                            match color {
                                "red" => number <= max_red,
                                "green" => number <= max_green,
                                "blue" => number <= max_blue,
                                _ => false,
                            }
                        })
                    });
    
                if possible {
                    Some(sum + index + 1)
                } else {
                    Some(sum)
                }
            })
            .context("")?;
    
        let sum = u32::try_from(sum)?;
        Ok(sum)
    }
    
    
    fn solve_part_two() -> Result<u32> {
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;
    
        let path = Path::new("src/inputs/day2.txt");
        let sum = read_to_string(path)?
            .lines()
            .enumerate()
            .try_fold(0, |sum, (index, line)| {
                let possible = line
                    .strip_prefix(&format!("Game {}: ", index + 1))?
                    .split_terminator("; ")
                    .all(|hand| {
                        hand.split_terminator(", ").all(|color_cube| {
                            let mut iter = color_cube.split_terminator(" ");
                            let number = if let Some(number) = iter.next() { number } else { return false; };
                            let number: u32 = if let Ok(number) = number.parse() { number } else { return false; };
                            let color = if let Some(color) = iter.next() { color} else { return false; };
    
                            match color {
                                "red" => number <= max_red,
                                "green" => number <= max_green,
                                "blue" => number <= max_blue,
                                _ => false,
                            }
                        })
                    });
    
                if possible {
                    Some(sum + index + 1)
                } else {
                    Some(sum)
                }
            })
            .context("")?;
    
        let sum = u32::try_from(sum)?;
        Ok(sum)
    }
}
