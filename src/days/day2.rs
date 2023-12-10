use crate::Solvable;

use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;

pub struct Day2 {}

impl Solvable for Day2 {
    fn get_day() -> u32 {
        2
    }

    fn solve_part_one(_: bool) -> Result<u32> {
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
                            let number = if let Some(number) = iter.next() {
                                number
                            } else {
                                return false;
                            };
                            let number: u32 = if let Ok(number) = number.parse() {
                                number
                            } else {
                                return false;
                            };
                            let color = if let Some(color) = iter.next() {
                                color
                            } else {
                                return false;
                            };

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

    fn solve_part_two(debug: bool) -> Result<u32> {
        let path = Path::new("src/inputs/day2.txt");
        read_to_string(path)?
            .lines()
            .enumerate()
            .try_fold(0, |sum, (index, line)| {
                let max_colors = line
                    .strip_prefix(&format!("Game {}: ", index + 1))?
                    .split_terminator("; ")
                    .try_fold((None::<u32>, None::<u32>, None::<u32>), |max_set, hand| {
                        let set = hand.split_terminator(", ").try_fold(
                            (None::<u32>, None::<u32>, None::<u32>),
                            |set, color_cube| {
                                let mut iter = color_cube.split_terminator(" ");
                                let number = iter.next()?.parse().ok()?;

                                let color = iter.next()?;

                                match color {
                                    "red" => Some((Some(number), set.1, set.2)),
                                    "green" => Some((set.0, Some(number), set.2)),
                                    "blue" => Some((set.0, set.1, Some(number))),
                                    _ => None,
                                }
                            },
                        )?;

                        let new_max_set = [set.0, set.1, set.2]
                            .iter()
                            .zip([max_set.0, max_set.1, max_set.2])
                            .map(|(color, max_color)| {
                                if let Some(color) = color {
                                    if let Some(max_color) = max_color {
                                        if max_color > *color {
                                            return Some(max_color);
                                        }
                                    }

                                    return Some(*color);
                                } else {
                                    return max_color;
                                }
                            })
                            .collect::<Vec<Option<u32>>>();

                        Some((new_max_set[0], new_max_set[1], new_max_set[2]))
                    })?;

                if debug {
                    println!("{:?}", max_colors);
                }

                if let (Some(max_red), Some(max_green), Some(max_blue)) = max_colors {
                    Some(sum + (max_red * max_green * max_blue))
                } else {
                    None
                }
            })
            .context("")
    }
}
