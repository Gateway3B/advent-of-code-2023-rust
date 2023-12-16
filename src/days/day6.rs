use crate::Solvable;

use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
struct RaceRecord {
    time: u64,
    distance: u64,
}

impl RaceRecord {
    pub fn margin_of_error(self: &Self) -> usize {
        (1..self.time)
            .into_iter()
            .map(|speed| speed * (self.time - speed))
            .filter(|distance| distance > &self.distance)
            .collect::<Vec<u64>>()
            .len()
    }

    pub fn margin_of_error_efficient(self: &Self) -> u64 {
        let start_index = (1..self.time)
            .into_iter()
            .find(|speed| {
                let distance = speed * (self.time - speed);
                distance > self.distance
            })
            .unwrap_or_default();

        self.time - (start_index * 2) + 1
    }
}

pub struct Day6 {}

impl Solvable for Day6 {
    fn get_day() -> u32 {
        6
    }

    fn solve_part_one(debug: bool) -> Result<i64> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let file_string = read_to_string(path)?;
        let mut lines = file_string.lines();

        let mut times = lines.next().context("")?.split_whitespace();
        times.next();
        let times = times
            .map(|time| time.parse().map_err(anyhow::Error::msg))
            .collect::<Result<Vec<u64>>>()?;

        let mut distances = lines.next().context("")?.split_whitespace();
        distances.next();
        let distances = distances
            .map(|time| time.parse().map_err(anyhow::Error::msg))
            .collect::<Result<Vec<u64>>>()?;

        let race_records = times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| RaceRecord { time, distance })
            .collect::<Vec<RaceRecord>>();

        if debug {
            println!("{:#?}", &race_records);
        }

        let margin_product = race_records
            .into_iter()
            .map(|race_record| race_record.margin_of_error())
            .fold(None, |product, margin| match product {
                Some(product) => Some(product * margin),
                None => Some(margin),
            })
            .context("Mult failed.")?;

        let margin_product = i64::try_from(margin_product)?;
        Ok(margin_product)
    }

    fn solve_part_two(debug: bool) -> Result<i64> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let file_string = read_to_string(path)?;
        let mut lines = file_string.lines();

        let mut time = lines.next().context("")?.split_whitespace();
        time.next();
        let time: u64 = time
            .fold(String::new(), |combined_string, number| {
                combined_string + number
            })
            .parse()?;

        let mut distance = lines.next().context("")?.split_whitespace();
        distance.next();
        let distance: u64 = distance
            .fold(String::new(), |combined_string, number| {
                combined_string + number
            })
            .parse()?;

        let race_record = RaceRecord { time, distance };

        if debug {
            println!("{:#?}", &race_record);
        }

        let margin_of_error = race_record.margin_of_error_efficient();

        let margin_of_error = i64::try_from(margin_of_error)?;
        Ok(margin_of_error)
    }
}
