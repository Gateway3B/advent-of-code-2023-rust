use crate::Solvable;

use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;

use std::collections::HashMap;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

pub struct Day8 {}

impl Solvable for Day8 {
    fn get_day() -> u32 {
        8
    }

    fn solve_part_one(debug: bool) -> Result<u32> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);
        let file_string = read_to_string(path)?;
        let mut lines = file_string.lines();
        let instructions = lines.next().context("No instructions")?.to_owned();

        lines.next().context("")?;

        let map = lines
            .try_fold(HashMap::new(), |mut map, line| {
                let (key, value) = line.split_once(" = (")?;
                let value = value.trim_end_matches(")");
                let value = value.split_once(", ")?;
                let value = (value.0.to_owned(), value.1.to_owned());

                map.insert(key.to_owned(), value);

                Some(map)
            })
            .context("Could not parse map")?;

        if debug {
            println!("{:#?}", instructions);
            println!("{:#?}", map);
        }

        let mut key = "AAA";
        let mut instructions_iter = instructions.chars();
        let mut direction = instructions_iter.next().context("")?;

        let mut step_count = 0;

        loop {
            let value = map.get(key).context("Map does not contain key")?;
            if direction == 'L' {
                key = &value.0;
            } else {
                key = &value.1;
            }

            step_count += 1;

            if key == "ZZZ" {
                break;
            }

            direction = if let Some(direction) = instructions_iter.next() {
                direction
            } else {
                instructions_iter = instructions.chars();
                instructions_iter.next().context("")?
            }
        }

        Ok(step_count)
    }

    fn solve_part_two(debug: bool) -> Result<u32> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);
        let file_string = read_to_string(path)?;
        let mut lines = file_string.lines();
        let instructions = lines.next().context("No instructions")?.to_owned();

        lines.next().context("")?;

        let map = lines
            .try_fold(HashMap::new(), |mut map, line| {
                let (key, value) = line.split_once(" = (")?;
                let value = value.trim_end_matches(")");
                let value = value.split_once(", ")?;
                let value = (value.0.to_owned(), value.1.to_owned());

                map.insert(key.to_owned(), value);

                Some(map)
            })
            .context("Could not parse map")?;

        // if debug {
        //     println!("{:#?}", instructions);
        //     println!("{:#?}", map);
        // }

        let keys = map.keys().fold(Vec::new(), |mut keys, key| {
            if key.ends_with("A") {
                keys.push(key);
            }
            keys
        });

        if debug {
            println!("{:#?}", keys);
        }

        let path_lengths = keys
            .into_iter()
            .map(|key| {
                let mut key = key;
                (|| {
                    let mut instructions_iter = instructions.chars();
                    let mut direction = instructions_iter.next().context("")?;

                    let mut step_count = 0;

                    loop {
                        let value = map.get(key).context("Map does not contain key")?;
                        if direction == 'L' {
                            key = &value.0;
                        } else {
                            key = &value.1;
                        }

                        step_count += 1;

                        if key.ends_with("Z") {
                            break;
                        }

                        direction = if let Some(direction) = instructions_iter.next() {
                            direction
                        } else {
                            instructions_iter = instructions.chars();
                            instructions_iter.next().context("")?
                        }
                    }

                    Ok(step_count)
                })()
            })
            .collect::<Result<Vec<u64>>>()?;

        let step_count = path_lengths
            .into_iter()
            .fold(None::<u64>, |lcm_res, length| {
                if let Some(lcm_res) = lcm_res {
                    Some(lcm(lcm_res, length))
                } else {
                    Some(length)
                }
            })
            .context("No least common multiple")?;

        println!("{}", step_count);

        Ok(0)
    }
}
