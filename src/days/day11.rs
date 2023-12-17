use crate::Solvable;

use anyhow::Result;
use std::fs::read_to_string;
use std::path::Path;

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    pub fn distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

pub struct Day11 {}

impl Solvable for Day11 {
    fn get_day() -> u32 {
        11
    }

    fn solve_part_one(debug: bool) -> Result<i64> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        // Initial parse
        let mut galaxies = Vec::new();
        let mut populated_columns = HashSet::new();
        let mut width = 0;

        read_to_string(path)?
            .lines()
            .enumerate()
            .fold(0, |vertical_expansion_count, (y, line)| {
                width = line.len();

                let row_has_galaxy =
                    line.chars()
                        .enumerate()
                        .fold(false, |row_has_galaxy, (x, char)| {
                            let is_galaxy = char == '#';
                            if is_galaxy {
                                galaxies.push(Galaxy {
                                    x,
                                    y: y + vertical_expansion_count,
                                });
                                populated_columns.insert(x);
                            }
                            row_has_galaxy || is_galaxy
                        });

                if row_has_galaxy {
                    vertical_expansion_count
                } else {
                    vertical_expansion_count + 1
                }
            });

        // Account for horizontal expansion
        let mut horizontal_expansion_counts = HashMap::new();
        (0..width).fold(0, |mut horizontal_expansion_count, x| {
            if !populated_columns.contains(&x) {
                horizontal_expansion_count += 1;
            }
            horizontal_expansion_counts.insert(x, horizontal_expansion_count);
            horizontal_expansion_count
        });

        if debug {
            println!("horizontal_expansion_counts");
            println!("{:#?}", horizontal_expansion_counts);
            println!("");
        }

        galaxies.iter_mut().for_each(|galaxy| {
            if horizontal_expansion_counts.contains_key(&galaxy.x) {
                if debug {
                    println!("Before: {:?}", galaxy);
                }

                galaxy.x += horizontal_expansion_counts.get(&galaxy.x).unwrap_or(&0);

                if debug {
                    println!("After: {:?}", galaxy);
                    println!("");
                }
            }
        });

        if debug {
            galaxies.iter().for_each(|galaxy| {
                println!("{:?}", galaxy);
            });
            println!("");
        }

        // Get sum of min dists
        let sum_of_min_dists =
            galaxies
                .iter()
                .enumerate()
                .fold(0, |sum, (index, starting_galaxy)| {
                    if debug {
                        println!("");
                    }
                    sum + galaxies
                        .iter()
                        .skip(index + 1)
                        .fold(0, |sum, ending_galaxy| {
                            let dist = starting_galaxy.distance(&ending_galaxy);
                            if debug {
                                println!("{:?} -> {:?} = {}", starting_galaxy, ending_galaxy, dist);
                            }
                            sum + dist
                        })
                });

        let sum_of_min_dists = i64::try_from(sum_of_min_dists)?;
        Ok(sum_of_min_dists)
    }

    fn solve_part_two(debug: bool) -> Result<i64> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let expansion_factor = 1_000_000;

        // Initial parse
        let mut galaxies = Vec::new();
        let mut populated_columns = HashSet::new();
        let mut width = 0;

        read_to_string(path)?
            .lines()
            .enumerate()
            .fold(0, |vertical_expansion_count, (y, line)| {
                width = line.len();

                let row_has_galaxy =
                    line.chars()
                        .enumerate()
                        .fold(false, |row_has_galaxy, (x, char)| {
                            let is_galaxy = char == '#';
                            if is_galaxy {
                                galaxies.push(Galaxy {
                                    x,
                                    y: y + vertical_expansion_count,
                                });
                                populated_columns.insert(x);
                            }
                            row_has_galaxy || is_galaxy
                        });

                if row_has_galaxy {
                    vertical_expansion_count
                } else {
                    vertical_expansion_count + expansion_factor - 1
                }
            });

        // Account for horizontal expansion
        let mut horizontal_expansion_counts = HashMap::new();
        (0..width).fold(0, |mut horizontal_expansion_count, x| {
            if !populated_columns.contains(&x) {
                horizontal_expansion_count += expansion_factor - 1;
            }
            horizontal_expansion_counts.insert(x, horizontal_expansion_count);
            horizontal_expansion_count
        });

        if debug {
            println!("horizontal_expansion_counts");
            println!("{:#?}", horizontal_expansion_counts);
            println!("");
        }

        galaxies.iter_mut().for_each(|galaxy| {
            if horizontal_expansion_counts.contains_key(&galaxy.x) {
                if debug {
                    println!("Before: {:?}", galaxy);
                }

                galaxy.x += horizontal_expansion_counts.get(&galaxy.x).unwrap_or(&0);

                if debug {
                    println!("After: {:?}", galaxy);
                    println!("");
                }
            }
        });

        if debug {
            galaxies.iter().for_each(|galaxy| {
                println!("{:?}", galaxy);
            });
            println!("");
        }

        // Get sum of min dists
        let sum_of_min_dists =
            galaxies
                .iter()
                .enumerate()
                .fold(0, |sum, (index, starting_galaxy)| {
                    if debug {
                        println!("");
                    }
                    sum + galaxies
                        .iter()
                        .skip(index + 1)
                        .fold(0, |sum, ending_galaxy| {
                            let dist = starting_galaxy.distance(&ending_galaxy);
                            if debug {
                                println!("{:?} -> {:?} = {}", starting_galaxy, ending_galaxy, dist);
                            }
                            sum + dist
                        })
                });

        let sum_of_min_dists = i64::try_from(sum_of_min_dists)?;
        Ok(sum_of_min_dists)
    }
}
