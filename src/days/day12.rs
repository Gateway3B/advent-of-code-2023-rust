use crate::Solvable;

use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;

use std::str::FromStr;
use strum_macros::{Display, EnumIs, EnumString};

#[derive(Copy, Clone, EnumString, EnumIs, Debug, Display)]
enum Spring {
    #[strum(serialize = ".")]
    Operational,
    #[strum(serialize = "#")]
    Damaged,
    #[strum(serialize = "?")]
    Unknown,
}

pub struct Day12 {}

impl Solvable for Day12 {
    fn get_day() -> u32 {
        12
    }

    fn solve_part_one(debug: bool) -> Result<i64> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let records = read_to_string(path)?
            .lines()
            .map(|line| {
                (|| {
                    let (springs, damaged_springs) = line.split_once(" ").context("")?;
                    let springs = springs
                        .chars()
                        .map(|char| {
                            (|| {
                                Spring::from_str(char.to_string().as_str())
                                    .map_err(anyhow::Error::msg)
                            })()
                        })
                        .collect::<Result<Vec<Spring>>>()?;

                    let damaged_springs = damaged_springs
                        .split_terminator(",")
                        .map(|char| (|| char.parse().map_err(anyhow::Error::msg))())
                        .collect::<Result<Vec<u32>>>()?;

                    Ok((springs, damaged_springs))
                })()
            })
            .collect::<Result<Vec<(Vec<Spring>, Vec<u32>)>>>()?;

        let sum = records.iter().fold(0, |sum, (springs, damaged_springs)| {
            let question_count = springs.iter().filter(|spring| spring.is_unknown()).count();
            let question_count = u32::try_from(question_count).unwrap_or(0);

            sum + (0..(2usize.pow(question_count)))
                .filter(|permutation_index| {
                    let mut success = true;
                    let mut springs = springs.clone();
                    springs
                        .iter_mut()
                        .filter(|spring| spring.is_unknown())
                        .enumerate()
                        .for_each(|(index, spring)| {
                            let index = u32::try_from(index).unwrap_or(0);

                            if permutation_index & 2usize.pow(index) > 0 {
                                *spring = Spring::Damaged;
                            } else {
                                *spring = Spring::Operational;
                            }
                        });

                    let mut damaged_springs_iter = damaged_springs.iter();
                    let damaged_count = springs.iter().fold(0, |damaged_count, spring| {
                        if spring.is_damaged() {
                            damaged_count + 1
                        } else {
                            if damaged_count > 0 {
                                let expected_count = damaged_springs_iter.next().unwrap_or(&0);
                                if expected_count != &damaged_count {
                                    success = false;
                                }
                            }
                            0
                        }
                    });

                    if damaged_count > 0 {
                        let expected_count = damaged_springs_iter.next().unwrap_or(&0);
                        if expected_count != &damaged_count {
                            success = false;
                        }
                    }

                    if let Some(_) = damaged_springs_iter.next() {
                        success = false;
                    }

                    if debug && success {
                        println!("{:?}", damaged_springs);
                        println!("{:?}\n", springs);
                    }

                    success
                })
                .count()
        });

        let sum = i64::try_from(sum)?;
        Ok(sum)
    }

    fn solve_part_two(debug: bool) -> Result<i64> {
        Self::solve_part_one(debug)
    }

    // fn solve_part_two(_debug: bool) -> Result<i64> {
    //     let path = format!("src/inputs/day{}.txt", Self::get_day());
    //     let path = Path::new(&path);

    //     let mut records = read_to_string(path)?
    //         .lines()
    //         .map(|line| {
    //             (|| {
    //                 let (springs, damaged_springs) = line.split_once(" ").context("")?;
    //                 let mut springs = springs
    //                     .chars()
    //                     .map(|char| {
    //                         (|| {
    //                             Spring::from_str(char.to_string().as_str())
    //                                 .map_err(anyhow::Error::msg)
    //                         })()
    //                     })
    //                     .collect::<Result<Vec<Spring>>>()?;

    //                 springs.push(Spring::Operational);

    //                 let damaged_springs = damaged_springs
    //                     .split_terminator(",")
    //                     .map(|char| (|| char.parse().map_err(anyhow::Error::msg))())
    //                     .collect::<Result<Vec<u32>>>()?;

    //                 Ok((springs, damaged_springs))
    //             })()
    //         })
    //         .collect::<Result<Vec<(Vec<Spring>, Vec<u32>)>>>()?;

    //     let sum = records.iter().fold(0, |sum, (springs, damaged_springs)| {
    //         let total_damage_count: usize =
    //             usize::try_from(damaged_springs.iter().sum::<u32>()).unwrap_or_default();
    //         let max_slide = springs.len() - total_damage_count - damaged_springs.len() + 1;

    //         let mut prev_damaged = Vec::new();
    //         springs.iter().fold(0, |prev_damaged_count, spring| {
    //             let mut damaged_count = prev_damaged_count;
    //             prev_damaged.push(damaged_count);

    //             if spring.is_damaged() {
    //                 damaged_count += 1;
    //             }
    //             damaged_count
    //         });

    //         damaged_springs.iter().fold(0, |curr_damaged_springs_count, damaged_springs_count| {
    //             springs.iter().enumerate().fold(0, |curr_damaged_springs_count, (spring_index, spring)| {
    //                 match spring {
    //                     Spring::Operational => {
    //                         0
    //                     },
    //                     Spring::Damaged => {
    //                         curr_damaged_springs_count + 1
    //                     },
    //                     Spring::Unknown => {

    //                     }
    //                 }
    //             })
    //         });
    //         0
    //     });

    //     let sum = i64::try_from(sum)?;
    //     Ok(sum)
    // }
}
