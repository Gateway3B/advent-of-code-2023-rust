use crate::Solvable;

use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;

use std::str::FromStr;
use strum_macros::{Display, EnumIs, EnumString};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, EnumString, EnumIs, Debug, Display, Hash)]
enum Rock {
    #[strum(serialize = ".")]
    Empty,
    #[strum(serialize = "#")]
    Cube,
    #[strum(serialize = "O")]
    Round,
}

#[derive(Debug, Hash)]
struct Platform {
    rocks: Vec<Vec<Rock>>,
}

impl Platform {
    fn display_grid(&self) {
        for row in self.rocks.iter() {
            for tile in row.iter() {
                print!("{}", tile);
            }
            println!("");
        }
        println!("");
    }

    fn get_rock(&self, x: usize, y: usize) -> Option<Rock> {
        self.rocks.get(y)?.get(x).map(|tile| tile.clone())
    }

    fn set_rock(&mut self, x: usize, y: usize, rock: Rock) -> Option<()> {
        let row = self.rocks.get_mut(y)?;
        row.push(rock);
        row.swap_remove(x);
        Some(())
    }

    fn roll_up(&mut self) -> Option<()> {
        let height = self.rocks.len();
        let width = self.rocks.get(0)?.len();

        for x in 0..width {
            'y: for y in 0..height {
                if self.get_rock(x, y)?.is_round() {
                    for yo in (0..y).rev() {
                        if !self.get_rock(x, yo)?.is_empty() {
                            self.set_rock(x, y, Rock::Empty);
                            self.set_rock(x, yo + 1, Rock::Round);
                            continue 'y;
                        }
                    }
                    self.set_rock(x, y, Rock::Empty);
                    self.set_rock(x, 0, Rock::Round);
                }
            }
        }

        Some(())
    }

    fn roll_left(&mut self) -> Option<()> {
        let height = self.rocks.len();
        let width = self.rocks.get(0)?.len();

        for y in 0..height {
            'x: for x in 0..width {
                if self.get_rock(x, y)?.is_round() {
                    for xo in (0..x).rev() {
                        if !self.get_rock(xo, y)?.is_empty() {
                            self.set_rock(x, y, Rock::Empty);
                            self.set_rock(xo + 1, y, Rock::Round);
                            continue 'x;
                        }
                    }
                    self.set_rock(x, y, Rock::Empty);
                    self.set_rock(0, y, Rock::Round);
                }
            }
        }

        Some(())
    }

    fn roll_down(&mut self) -> Option<()> {
        let height = self.rocks.len();
        let width = self.rocks.get(0)?.len();

        for x in 0..width {
            'y: for y in (0..height).rev() {
                if self.get_rock(x, y)?.is_round() {
                    for yo in (y + 1)..height {
                        if !self.get_rock(x, yo)?.is_empty() {
                            self.set_rock(x, y, Rock::Empty);
                            self.set_rock(x, yo - 1, Rock::Round);
                            continue 'y;
                        }
                    }
                    self.set_rock(x, y, Rock::Empty);
                    self.set_rock(x, height - 1, Rock::Round);
                }
            }
        }

        Some(())
    }

    fn roll_right(&mut self) -> Option<()> {
        let height = self.rocks.len();
        let width = self.rocks.get(0)?.len();

        for y in 0..height {
            'x: for x in (0..width).rev() {
                if self.get_rock(x, y)?.is_round() {
                    for xo in (x + 1)..width {
                        if !self.get_rock(xo, y)?.is_empty() {
                            self.set_rock(x, y, Rock::Empty);
                            self.set_rock(xo - 1, y, Rock::Round);
                            continue 'x;
                        }
                    }
                    self.set_rock(x, y, Rock::Empty);
                    self.set_rock(width - 1, y, Rock::Round);
                }
            }
        }

        Some(())
    }
    fn cycle(&mut self) -> Result<()> {
        self.roll_up().context("Error rolling up.")?;
        self.roll_left().context("Error rolling left.")?;
        self.roll_down().context("Error rolling down.")?;
        self.roll_right().context("Error rolling right.")?;
        Ok(())
    }

    fn calc_top_load(self) -> Option<usize> {
        let height = self.rocks.len();
        let width = self.rocks.get(0)?.len();

        let mut total_load = 0;
        for y in 0..height {
            for x in 0..width {
                if self.get_rock(x, y)?.is_round() {
                    total_load += height - y;
                }
            }
        }

        Some(total_load)
    }

    fn calculate_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

pub struct Day14 {}

impl Solvable for Day14 {
    fn get_day() -> u32 {
        14
    }

    fn solve_part_one(debug: bool) -> Result<i64> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let mut platform = Platform { rocks: Vec::new() };
        for line in read_to_string(path)?.lines() {
            let mut rock_row = Vec::new();
            for char in line.chars() {
                let rock = Rock::from_str(char.to_string().as_str())?;
                rock_row.push(rock);
            }
            platform.rocks.push(rock_row);
        }

        if debug {
            platform.display_grid();
        }

        platform.roll_up().context("Error rolling up.")?;

        if debug {
            platform.display_grid();
        }

        let total_load = platform
            .calc_top_load()
            .context("Issue calculating total load.")?;

        let total_load = i64::try_from(total_load).context("")?;

        Ok(total_load)
    }

    fn solve_part_two(debug: bool) -> Result<i64> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let mut platform = Platform { rocks: Vec::new() };
        for line in read_to_string(path)?.lines() {
            let mut rock_row = Vec::new();
            for char in line.chars() {
                let rock = Rock::from_str(char.to_string().as_str())?;
                rock_row.push(rock);
            }
            platform.rocks.push(rock_row);
        }

        if debug {
            platform.display_grid();
        }

        let mut seen = Vec::new();
        let mut index = 0;
        while index < 1_000_000_000 {
            let hash = platform.calculate_hash();
            if let Some(seen_index) = seen.iter().position(|seen| seen == &hash) {
                let cycle_length = index - seen_index;
                index = (1_000_000_000 - index) % cycle_length;
                break;
            }

            seen.push(hash);
            platform.cycle()?;
            index += 1;
        }

        if debug {
            println!("{}", index);
        }

        for _ in 0..index {
            platform.cycle()?;
        }

        if debug {
            platform.display_grid();
        }

        let total_load = platform
            .calc_top_load()
            .context("Issue calculating total load.")?;

        let total_load = i64::try_from(total_load).context("")?;

        Ok(total_load)
    }
}
