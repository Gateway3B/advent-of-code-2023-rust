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

#[derive(PartialEq, Debug)]
enum SeedRangeStatus {
    Completed(SeedRange),
    Uncompleted(SeedRange),
}

impl SeedRangeStatus {
    pub fn extract(self: Self) -> SeedRange {
        match self {
            SeedRangeStatus::Completed(seed_range) => seed_range,
            SeedRangeStatus::Uncompleted(seed_range) => seed_range,
        }
    }
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

    pub fn split_map(self: &Self, seed_range: SeedRange) -> Vec<SeedRangeStatus> {
        let index_start = seed_range.start - self.source_start;
        let start_overlap = index_start > 0 && index_start < self.length;

        let index_end = seed_range.end - self.source_start;
        let end_overlap = index_end > 0 && index_end < self.length;

        let total_overlap = index_start < 0 && index_end > self.length;

        if total_overlap {
            return vec![
                SeedRangeStatus::Uncompleted(SeedRange {
                    start: self.source_start,
                    end: self.source_start - 1,
                }),
                SeedRangeStatus::Completed(SeedRange {
                    start: self.destination_start,
                    end: self.destination_start + self.length - 1,
                }),
                SeedRangeStatus::Uncompleted(SeedRange {
                    start: self.source_start + self.length,
                    end: seed_range.end,
                }),
            ];
        }

        match (start_overlap, end_overlap) {
            (true, true) => vec![SeedRangeStatus::Completed(SeedRange {
                start: self.destination_start + index_start,
                end: self.destination_start + index_end,
            })],
            (true, false) => vec![
                SeedRangeStatus::Completed(SeedRange {
                    start: self.destination_start + index_start,
                    end: self.destination_start + self.length - 1,
                }),
                SeedRangeStatus::Uncompleted(SeedRange {
                    start: self.source_start + self.length,
                    end: seed_range.end,
                }),
            ],
            (false, true) => vec![
                SeedRangeStatus::Uncompleted(SeedRange {
                    start: seed_range.start,
                    end: self.source_start - 1,
                }),
                SeedRangeStatus::Completed(SeedRange {
                    start: self.destination_start,
                    end: self.destination_start + index_end,
                }),
            ],
            (false, false) => vec![SeedRangeStatus::Uncompleted(seed_range)],
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

    pub fn split_map(self: &Self, seed_range: SeedRange) -> Vec<SeedRange> {
        self.ranges
            .iter()
            .fold(
                vec![SeedRangeStatus::Uncompleted(seed_range)],
                |seed_ranges, range| {
                    let (uncompleted, mut completed): (Vec<SeedRangeStatus>, Vec<SeedRangeStatus>) =
                        seed_ranges.into_iter().partition(|seed_range| {
                            matches!(seed_range, SeedRangeStatus::Uncompleted(_))
                        });

                    let new_seed_ranges =
                        uncompleted
                            .into_iter()
                            .fold(Vec::new(), |mut seed_ranges, seed_range| {
                                seed_ranges.extend(range.split_map(seed_range.extract()));
                                seed_ranges
                            });

                    completed.extend(new_seed_ranges);

                    completed
                },
            )
            .into_iter()
            .map(|seed_range_status| seed_range_status.extract())
            .collect()
    }
}

struct Maps {
    maps: Vec<Map>,
}

#[derive(Debug, PartialEq)]
struct SeedRange {
    start: i64,
    end: i64,
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

        let seed_ranges = seeds
            .chunks(2)
            .map(|slice| SeedRange {
                start: slice[0],
                end: slice[0] + slice[1],
            })
            .collect::<Vec<SeedRange>>();

        if debug {
            println!("{:?}", &seed_ranges);
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
                let destination_start = range_iter
                    .next()
                    .context("No destination start number.")?
                    .parse()?;
                let source_start = range_iter
                    .next()
                    .context("No source start number.")?
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
            println!("{:#?}", &maps.maps);
        }

        let mapped_seed_ranges = seed_ranges
            .into_iter()
            .map(|seed_range| {
                if debug {
                    println!("Seed Range: {:?}", &seed_range);
                }
                maps.maps
                    .iter()
                    .fold(vec![seed_range], |mapped_seed_ranges, map| {
                        let new_mapped_seed_ranges = mapped_seed_ranges.into_iter().fold(
                            Vec::new(),
                            |mut new_ranges, range| {
                                let mut split_ranges = map.split_map(range);
                                new_ranges.append(&mut split_ranges);
                                new_ranges
                            },
                        );
                        if debug {
                            println!("\t{:?}", new_mapped_seed_ranges);
                        }
                        new_mapped_seed_ranges
                    })
            })
            .collect::<Vec<Vec<SeedRange>>>();

        let mapped_seed_ranges =
            mapped_seed_ranges
                .into_iter()
                .fold(Vec::new(), |mut new_seed_ranges, seed_ranges| {
                    new_seed_ranges.extend(seed_ranges);
                    new_seed_ranges
                });

        if debug {
            println!("{:?}", &mapped_seed_ranges);
        }

        let min = mapped_seed_ranges
            .into_iter()
            .map(|seed_range| seed_range.start)
            .filter(|range_start| range_start > &0)
            .min()
            .context("Couldn't find min.")?;

        let min = u32::try_from(min)?;
        Ok(min)
    }
}
