use crate::Solvable;

use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;

use std::collections::HashMap;

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

        let mut keys = map.keys().fold(Vec::new(), |mut keys, key| {
            if key.ends_with("A") {
                keys.push(key);
            }
            keys
        });

        if debug {
            println!("{:#?}", keys);
        }

        let mut instructions_iter = instructions.chars();
        let mut direction = instructions_iter.next().context("")?;

        let mut step_count = 0;

        loop {
            keys = keys
                .iter()
                .map(|key| {
                    (|| {
                        let value = map.get(*key)?;
                        if direction == 'L' {
                            Some(&value.0)
                        } else {
                            Some(&value.1)
                        }
                    })()
                })
                .collect::<Option<Vec<&String>>>()
                .context("")?;

            step_count += 1;

            if keys.iter().all(|key| key.ends_with("Z")) {
                break;
            }

            if debug {
                println!("{:#?}", keys);
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
}
