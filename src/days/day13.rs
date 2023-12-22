use crate::Solvable;

use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;

use std::str::FromStr;
use strum_macros::{Display, EnumIs, EnumString};

#[derive(Copy, Clone, PartialEq, EnumString, EnumIs, Debug, Display)]
enum Tile {
    #[strum(serialize = ".")]
    Ash,
    #[strum(serialize = "#")]
    Rocks,
}

#[derive(Debug)]
struct Ground {
    tiles: Vec<Vec<Tile>>,
}

impl Ground {
    // fn display_grid(&self, highlights: Vec<(usize, usize)>) {
    //     for (y, row) in self.tiles.iter().enumerate() {
    //         for (x, tile) in row.iter().enumerate() {
    //             if highlights.iter().any(|(hx, hy)| hx == &x && hy == &y) {
    //                 match tile {
    //                     Tile::Ash => print!(";"),
    //                     Tile::Rocks => print!("$")
    //                 }
    //             }
    //             print!("{}", tile);
    //         }
    //         println!("");
    //     }
    //     println!("");
    // }
    
    fn get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        self.tiles.get(y)?.get(x).map(|tile| tile.clone())
    }

    fn vertical_mirror(&self) -> Option<usize> {
        let height = self.tiles.len();
        let width = self.tiles.get(0)?.len();

        let mut res = None;
        'col: for x in 1..width {
            for y in 0..height {
                for (xl, xr) in (0..x).rev().zip(x..width) {
                    // println!("({}, {}):{} - {} : {}", xl, xr, y, self.get_tile(xl, y)?, self.get_tile(xr, y)?);

                    if self.get_tile(xl, y)? != self.get_tile(xr, y)? {
                        continue 'col;
                    }
                }
            }
            res = Some(x);
            break 'col;
        }

        res
    }

    fn horizontal_mirror(&self) -> Option<usize> {
        let height = self.tiles.len();
        let width = self.tiles.get(0)?.len();

        let mut res = None;
        'row: for y in 1..height {
            for x in 0..width {
                for (yu, yd) in (0..y).rev().zip(y..height) {
                    // println!("{}:({}, {}) - {} : {}", x, yu, yd, self.get_tile(x, yu)?, self.get_tile(x, yd)?);
                    // self.display_grid(vec![(x, yu), (x, yd)]);

                    if self.get_tile(x, yu)? != self.get_tile(x, yd)? {
                        continue 'row;
                    }
                }
            }
            res = Some(y);
            break 'row;
        }

        res
    }

    fn vertical_mirror_smudges(&self) -> Option<usize> {
        let height = self.tiles.len();
        let width = self.tiles.get(0)?.len();

        let mut res = None;
        'col: for x in 1..width {
            let mut smudge = false;

            for y in 0..height {
                for (xl, xr) in (0..x).rev().zip(x..width) {
                    // println!("({}, {}):{} - {} : {}", xl, xr, y, self.get_tile(xl, y)?, self.get_tile(xr, y)?);

                    if self.get_tile(xl, y)? != self.get_tile(xr, y)? {
                        if smudge {
                            continue 'col;
                        } else {
                            smudge = true
                        }
                    }
                }
            }

            if smudge {
                res = Some(x);
                break 'col;
            }
        }

        res
    }

    fn horizontal_mirror_smudges(&self) -> Option<usize> {
        let height = self.tiles.len();
        let width = self.tiles.get(0)?.len();

        let mut res = None;
        'row: for y in 1..height {
            let mut smudge = false;

            for x in 0..width {
                for (yu, yd) in (0..y).rev().zip(y..height) {
                    // println!("{}:({}, {}) - {} : {}", x, yu, yd, self.get_tile(x, yu)?, self.get_tile(x, yd)?);
                    // self.display_grid(vec![(x, yu), (x, yd)]);

                    if self.get_tile(x, yu)? != self.get_tile(x, yd)? {
                        if smudge {
                            continue 'row;
                        } else {
                            smudge = true
                        }
                    }
                }
            }

            if smudge {
                res = Some(y);
                break 'row;
            }
        }

        res
    }
}

pub struct Day13 {}

impl Solvable for Day13 {
    fn get_day() -> u32 {
        13
    }

    fn solve_part_one(debug: bool) -> Result<i64> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let patterns: Vec<Ground> = read_to_string(path)?
            .lines()
            .try_fold(Vec::<Ground>::new(), |mut patterns, line| {
                if line.is_empty() {
                    patterns.push(Ground { tiles: Vec::new() });
                    return Ok(patterns);
                }
                if patterns.len() == 0 {
                    patterns.push(Ground { tiles: Vec::new() });
                }

                let pattern = patterns.last_mut().context("No last pattern")?;
                let pattern_line = line
                    .chars()
                    .map(|char| {
                        Tile::from_str(char.to_string().as_str()).map_err(anyhow::Error::msg)
                    })
                    .collect::<std::result::Result<Vec<Tile>, anyhow::Error>>()?;
                pattern.tiles.push(pattern_line);

                return Ok(patterns);
            })
            .map_err(anyhow::Error::msg::<anyhow::Error>)?;

        if debug {
            println!("{:?}", patterns);
        }

        let sum: usize = patterns
            .iter()
            .map(|ground| {
                let columns_left = ground.vertical_mirror().unwrap_or_default();
                let rows_above = ground.horizontal_mirror().unwrap_or_default();

                columns_left + (100 * rows_above)
            })
            .sum();

        let sum = i64::try_from(sum)?;
        Ok(sum)
    }

    fn solve_part_two(debug: bool) -> Result<i64> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let patterns: Vec<Ground> = read_to_string(path)?
            .lines()
            .try_fold(Vec::<Ground>::new(), |mut patterns, line| {
                if line.is_empty() {
                    patterns.push(Ground { tiles: Vec::new() });
                    return Ok(patterns);
                }
                if patterns.len() == 0 {
                    patterns.push(Ground { tiles: Vec::new() });
                }

                let pattern = patterns.last_mut().context("No last pattern")?;
                let pattern_line = line
                    .chars()
                    .map(|char| {
                        Tile::from_str(char.to_string().as_str()).map_err(anyhow::Error::msg)
                    })
                    .collect::<std::result::Result<Vec<Tile>, anyhow::Error>>()?;
                pattern.tiles.push(pattern_line);

                return Ok(patterns);
            })
            .map_err(anyhow::Error::msg::<anyhow::Error>)?;

        if debug {
            println!("{:?}", patterns);
        }

        let sum: usize = patterns
            .iter()
            .map(|ground| {
                let columns_left = ground.vertical_mirror_smudges().unwrap_or_default();
                let rows_above = ground.horizontal_mirror_smudges().unwrap_or_default();

                columns_left + (100 * rows_above)
            })
            .sum();

        let sum = i64::try_from(sum)?;
        Ok(sum)
    }
}
