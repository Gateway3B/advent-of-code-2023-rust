use crate::Solvable;

use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;

use std::collections::HashMap;

pub struct Day15 {}

impl Solvable for Day15 {
    fn get_day() -> u32 {
        15
    }

    fn solve_part_one(debug: bool) -> Result<i64> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let sum = read_to_string(path)?
            .lines()
            .next()
            .context("")?
            .split(",")
            .fold(0, |sum, chars| {
                let hash = chars
                    .chars()
                    .fold(0, |hash, char| ((hash + char as u64) * 17) % 256);

                let hash = i64::try_from(hash).unwrap_or_default();

                sum + hash
            });

        Ok(sum)
    }

    fn solve_part_two(debug: bool) -> Result<i64> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let mut boxes: HashMap<u32, Vec<(String, u32)>> = HashMap::new();

        read_to_string(path)?
            .lines()
            .next()
            .context("")?
            .split(",")
            .try_for_each(|chars| {
                let operation_index = chars
                    .chars()
                    .position(|char| char == '=' || char == '-')
                    .unwrap();

                let label = chars.chars().take(operation_index).collect();

                let hash = chars
                    .chars()
                    .take(operation_index)
                    .fold(0, |hash, char| ((hash + char as u32) * 17) % 256);

                let operation = chars
                    .chars()
                    .skip(operation_index)
                    .take(1)
                    .collect::<Vec<char>>()
                    .first()
                    .context("No operation")?
                    .clone();

                let curr_box = if let Some(curr_box) = boxes.get_mut(&hash) {
                    curr_box
                } else {
                    boxes.insert(hash.clone(), Vec::new());
                    boxes.get_mut(&hash).unwrap()
                };

                let existing_lens = curr_box
                    .iter()
                    .enumerate()
                    .find(|eval_box| eval_box.1 .0 == label)
                    .and_then(|lens| Some((lens.0, lens.1 .1)));

                let new_lens_power = chars
                    .chars()
                    .skip(operation_index + 1)
                    .take(1)
                    .collect::<Vec<char>>()
                    .first()
                    .and_then(|power| power.to_digit(10))
                    .context("");

                if debug {
                    println!("{}{}{:?}", label, operation, new_lens_power);
                }

                if let Some(existing_lens) = existing_lens {
                    if let Some(lens) = curr_box.get_mut(existing_lens.0) {
                        if operation == '=' {
                            lens.1 = new_lens_power?;
                        }

                        if operation == '-' {
                            curr_box.remove(existing_lens.0);
                        }
                    }
                } else {
                    if operation == '=' {
                        curr_box.push((label, new_lens_power?));
                    }
                }

                if curr_box.len() == 0 {
                    boxes.remove(&hash);
                }

                // if debug {
                //     println!("{:?}", boxes);
                // }

                Ok::<(), anyhow::Error>(())
            })?;

        if debug {
            println!("{:?}", boxes);
        }

        let sum = boxes.into_iter().fold(0, |acc, curr_box| {
            let box_pos = curr_box.0 + 1;
            let box_power = curr_box.1.into_iter().enumerate().fold(0, |acc, lens| {
                let lens_pos = lens.0 + 1;
                let lens_power = lens.1 .1;
                (lens_pos as u32 * lens_power) + acc
            });
            (box_pos as u32 * box_power) + acc
        });

        let sum = i64::try_from(sum).context("")?;

        Ok(sum)
    }
}
