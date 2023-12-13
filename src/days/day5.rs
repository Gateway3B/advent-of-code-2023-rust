use crate::Solvable;

use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
struct Range {
    source_start: i64,
    destination_start: i64,
    length: i64,
}

impl Range {
    pub fn map(self: &Self, input: i64) -> Option<i64> {
        let index = input - self.destination_start;
        if index < 0 || index > self.length {
            return None;
        } else {
            return Some(self.source_start + index);
        }
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    pub fn map(self: &Self, input: i64) -> i64 {
        self.ranges
            .iter()
            .find_map(|range| range.map(input))
            .unwrap_or(input)
    }
}

struct Maps {
    maps: Vec<Map>,
}

pub struct Day5 {}

impl Solvable for Day5 {
    fn get_day() -> u32 {
        5
    }

    fn solve_part_one(debug: bool) -> Result<u32> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let mut maps = Maps { maps: Vec::new() };

        let file_string = read_to_string(path)?;
        let mut lines = file_string.lines();
        let seeds = lines
            .next()
            .context("File empty.")?
            .strip_prefix("seeds: ")
            .context("No seeds.")?
            .split_whitespace()
            .map(|number| number.parse().map_err(anyhow::Error::msg))
            .collect::<Result<Vec<i64>>>()?;

        if debug {
            println!("{:?}", seeds);
        }

        let mut curr_map = Map { ranges: Vec::new() };
        loop {
            if let Some(mut line) = lines.next() {
                if line.is_empty() {
                    if curr_map.ranges.len() > 0 {
                        maps.maps.push(curr_map);
                    }
                    line = if let Some(_) = lines.next() {
                        if let Some(line) = lines.next() {
                            line
                        } else {
                            break;
                        }
                    } else {
                        break;
                    };
                    curr_map = Map { ranges: Vec::new() };
                }

                let mut range_iter = line.split_whitespace();
                let source_start = range_iter
                    .next()
                    .context("No source start number.")?
                    .parse()?;
                let destination_start = range_iter
                    .next()
                    .context("No destination start number.")?
                    .parse()?;
                let length = range_iter.next().context("No length number.")?.parse()?;

                let range = Range {
                    source_start,
                    destination_start,
                    length,
                };

                curr_map.ranges.push(range);
            } else {
                maps.maps.push(curr_map);
                break;
            }
        }

        if debug {
            println!("{:#?}", maps.maps);
        }

        let mapped_seeds = seeds
            .iter()
            .map(|seed| {
                if debug {
                    println!("Seed Input: {}", seed);
                }
                maps.maps.iter().fold(*seed, |seed, map| {
                    let mapped_seed = map.map(seed);
                    if debug {
                        println!("\t{}", mapped_seed);
                    }
                    mapped_seed
                })
            })
            .collect::<Vec<i64>>();

        if debug {
            println!("{:?}", &mapped_seeds);
        }

        let min = *mapped_seeds.iter().min().context("Couldn't find min.")?;

        let min = u32::try_from(min)?;
        Ok(min)
    }

    fn solve_part_two(debug: bool) -> Result<u32> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let mut maps = Maps { maps: Vec::new() };

        let file_string = read_to_string(path)?;
        let mut lines = file_string.lines();
        let seeds = lines
            .next()
            .context("File empty.")?
            .strip_prefix("seeds: ")
            .context("No seeds.")?
            .split_whitespace()
            .map(|number| number.parse().map_err(anyhow::Error::msg))
            .collect::<Result<Vec<i64>>>()?;

        let seeds = seeds.chunks(2).fold(Vec::new(), |mut seeds, slice| {
            seeds.extend((slice[0]..slice[0] + slice[1]).collect::<Vec<i64>>());
            seeds
        });

        if debug {
            println!("{:?}", seeds);
        }

        let mut curr_map = Map { ranges: Vec::new() };
        loop {
            if let Some(mut line) = lines.next() {
                if line.is_empty() {
                    if curr_map.ranges.len() > 0 {
                        maps.maps.push(curr_map);
                    }
                    line = if let Some(_) = lines.next() {
                        if let Some(line) = lines.next() {
                            line
                        } else {
                            break;
                        }
                    } else {
                        break;
                    };
                    curr_map = Map { ranges: Vec::new() };
                }

                let mut range_iter = line.split_whitespace();
                let source_start = range_iter
                    .next()
                    .context("No source start number.")?
                    .parse()?;
                let destination_start = range_iter
                    .next()
                    .context("No destination start number.")?
                    .parse()?;
                let length = range_iter.next().context("No length number.")?.parse()?;

                let range = Range {
                    source_start,
                    destination_start,
                    length,
                };

                curr_map.ranges.push(range);
            } else {
                maps.maps.push(curr_map);
                break;
            }
        }

        if debug {
            println!("{:#?}", maps.maps);
        }

        let mapped_seeds = seeds
            .iter()
            .map(|seed| {
                if debug {
                    println!("Seed Input: {}", seed);
                }
                maps.maps.iter().fold(*seed, |seed, map| {
                    let mapped_seed = map.map(seed);
                    if debug {
                        println!("\t{}", mapped_seed);
                    }
                    mapped_seed
                })
            })
            .collect::<Vec<i64>>();

        if debug {
            println!("{:?}", &mapped_seeds);
        }

        let min = *mapped_seeds.iter().min().context("Couldn't find min.")?;

        let min = u32::try_from(min)?;
        Ok(min)
    }
}
