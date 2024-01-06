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
                let label = chars.chars().take(2).collect();
                let hash = chars
                    .chars()
                    .take(2)
                    .fold(0, |hash, char| ((hash + char as u32) * 17) % 256);

                if Some(lenses) = boxes.get_mut(&hash)) {
                    
                } else {
                    
                }
                lenses.(|(label, _)| {});

                Ok(())
            })?;

        Ok(sum)
    }
}
