use crate::Solvable;

use anyhow::Result;
use std::fs::read_to_string;
use std::path::Path;

pub struct Day9 {}

impl Solvable for Day9 {
    fn get_day() -> u32 {
        9
    }

    fn solve_part_one(debug: bool) -> Result<u32> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let reports = read_to_string(path)?
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|value| value.parse().map_err(anyhow::Error::msg))
                    .collect::<Result<Vec<i32>>>()
            })
            .collect::<Result<Vec<Vec<i32>>>>()?;

        let extrapolated_sum: i32 = reports
            .into_iter()
            .map(|report| {
                let mut report_diffs = Vec::new();
                report_diffs.push(report);

                loop {
                    let mut report_diff = Vec::new();

                    report_diffs
                        .last()
                        .unwrap()
                        .iter()
                        .fold(None, |prev_value, curr_value| {
                            if let Some(prev_value) = prev_value {
                                report_diff.push(curr_value - prev_value);
                            }
                            Some(curr_value)
                        });

                    if debug {
                        println!("{:?}", report_diff);
                    }

                    let value_sum: i32 = report_diff.iter().sum();

                    report_diffs.push(report_diff);

                    if value_sum == 0 {
                        break;
                    }
                }

                report_diffs.iter().fold(0, |prev, report_diff| {
                    prev + report_diff.last().unwrap_or(&0)
                })
            })
            .sum();

        let extrapolated_sum = u32::try_from(extrapolated_sum)?;

        Ok(extrapolated_sum)
    }

    fn solve_part_two(debug: bool) -> Result<u32> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let reports = read_to_string(path)?
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|value| value.parse().map_err(anyhow::Error::msg))
                    .collect::<Result<Vec<i32>>>()
            })
            .collect::<Result<Vec<Vec<i32>>>>()?;

        let extrapolated_values = reports
            .into_iter()
            .map(|report| {
                let mut report_diffs = Vec::new();
                report_diffs.push(report);

                loop {
                    let mut report_diff = Vec::new();

                    report_diffs
                        .last()
                        .unwrap()
                        .iter()
                        .fold(None, |prev_value, curr_value| {
                            if let Some(prev_value) = prev_value {
                                report_diff.push(curr_value - prev_value);
                            }
                            Some(curr_value)
                        });

                    if debug {
                        println!("{:?}", report_diff);
                    }

                    let value_sum: i32 = report_diff.iter().sum();

                    if value_sum == 0 {
                        break;
                    }

                    report_diffs.push(report_diff);
                }

                report_diffs.iter().rev().fold(0, |prev, report_diff| {
                    report_diff.first().unwrap_or(&0) - prev
                })
            })
            .collect::<Vec<i32>>();

        if debug {
            println!("{:?}", extrapolated_values)
        }

        let extrapolated_sum: i32 = extrapolated_values.iter().sum();

        println!("{}", extrapolated_sum);

        Ok(0)
    }
}
