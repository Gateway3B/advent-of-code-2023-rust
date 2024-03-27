use crate::Solvable;

use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use strum_macros::{Display, EnumIs, EnumString};

use std::collections::HashMap;

#[derive(Copy, Clone, EnumString, EnumIs, Debug, Display, Hash)]
enum Tile {
    #[strum(serialize = ".")]
    Empty,
    #[strum(serialize = "-")]
    HorizontalSplitter,
    #[strum(serialize = "|")]
    VerticalSplitter,
    #[strum(serialize = "/")]
    ForwardMirror,
    #[strum(serialize = r#"\"#)]
    BackwardMirror,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Coord {
    x: usize,
    y: usize,
}

fn new_coord(coord: Coord, direction: Direction) -> Option<Coord> {
    match direction {
        Direction::Left => coord
            .x
            .checked_sub(1)
            .and_then(|x| Some(Coord { x, y: coord.y })),
        Direction::Right => coord
            .x
            .checked_add(1)
            .and_then(|x| Some(Coord { x, y: coord.y })),
        Direction::Up => coord
            .y
            .checked_sub(1)
            .and_then(|y| Some(Coord { x: coord.x, y })),
        Direction::Down => coord
            .y
            .checked_add(1)
            .and_then(|y| Some(Coord { x: coord.x, y })),
    }
}

fn get_energized_tiles(mut grid: Vec<Vec<(Tile, bool)>>, coord: Coord, direction: Direction) {
    let mut row = match grid.get_mut(coord.y) {
        None => return,
        Some(row) => row,
    };

    let mut tile = match row.get_mut(coord.x) {
        None => return,
        Some(tile) => tile,
    };

    tile.1 = true;

    match tile.0 {
        Tile::Empty => {
            if let Some(new_coord) = new_coord(coord, direction) {
                get_energized_tiles(grid, new_coord, direction);
            }
        }
        Tile::BackwardMirror => {
            let new_direction = match direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };

            if let Some(new_coord) = new_coord(coord, new_direction) {
                get_energized_tiles(grid, new_coord, new_direction);
            }
        }
        Tile::ForwardMirror => {
            let new_direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            };

            if let Some(new_coord) = new_coord(coord, new_direction) {
                get_energized_tiles(grid, new_coord, new_direction);
            }
        }
        Tile::VerticalSplitter => {
            match direction {
                Direction::Down | Direction::Up => {
                    if let Some(new_coord) = new_coord(coord, new_direction) {
                        get_energized_tiles(grid, new_coord, new_direction);
                    }
                    if let Some(new_coord) = new_coord(coord, new_direction) {
                        get_energized_tiles(grid, new_coord, new_direction);
                    }
                }
                Direction::Left | Direction::Right => {
                    if let Some(new_coord) = new_coord(coord, new_direction) {
                        get_energized_tiles(grid, new_coord, new_direction);
                    }
                }
            }
            if let Some(new_coord) = new_coord(coord, new_direction) {
                get_energized_tiles(grid, new_coord, new_direction);
            }
        }
    }
}

pub struct Day16 {}

impl Solvable for Day16 {
    fn get_day() -> u32 {
        16
    }

    fn solve_part_one(debug: bool) -> Result<i64> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let grid = read_to_string(path)?
            .lines()
            .fold(Vec::new(), |grid, chars| {
                let row = chars.chars().fold(Vec::new(), |row, char| {
                    let tile = Tile::from_str(char.to_string().as_str()).unwrap();
                    row.push((tile, false));
                    row
                });

                grid.push(row);
                grid
            });
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
                let operation_index = chars
                    .chars()
                    .position(|char| char == '=' || char == '-')
                    .unwrap();

                let label = chars.chars().take(operation_index).collect();

                let hash = chars
                    .chars()
                    .take(operation_index)
                    .fold(0, |hash, char| ((hash + char as u32) * 17) % 256);

                let operation = chars
                    .chars()
                    .skip(operation_index)
                    .take(1)
                    .collect::<Vec<char>>()
                    .first()
                    .context("No operation")?
                    .clone();

                let curr_box = if let Some(curr_box) = boxes.get_mut(&hash) {
                    curr_box
                } else {
                    boxes.insert(hash.clone(), Vec::new());
                    boxes.get_mut(&hash).unwrap()
                };

                let existing_lens = curr_box
                    .iter()
                    .enumerate()
                    .find(|eval_box| eval_box.1 .0 == label)
                    .and_then(|lens| Some((lens.0, lens.1 .1)));

                let new_lens_power = chars
                    .chars()
                    .skip(operation_index + 1)
                    .take(1)
                    .collect::<Vec<char>>()
                    .first()
                    .and_then(|power| power.to_digit(10))
                    .context("");

                if debug {
                    println!("{}{}{:?}", label, operation, new_lens_power);
                }

                if let Some(existing_lens) = existing_lens {
                    if let Some(lens) = curr_box.get_mut(existing_lens.0) {
                        if operation == '=' {
                            lens.1 = new_lens_power?;
                        }

                        if operation == '-' {
                            curr_box.remove(existing_lens.0);
                        }
                    }
                } else {
                    if operation == '=' {
                        curr_box.push((label, new_lens_power?));
                    }
                }

                if curr_box.len() == 0 {
                    boxes.remove(&hash);
                }

                // if debug {
                //     println!("{:?}", boxes);
                // }

                Ok::<(), anyhow::Error>(())
            })?;

        if debug {
            println!("{:?}", boxes);
        }

        let sum = boxes.into_iter().fold(0, |acc, curr_box| {
            let box_pos = curr_box.0 + 1;
            let box_power = curr_box.1.into_iter().enumerate().fold(0, |acc, lens| {
                let lens_pos = lens.0 + 1;
                let lens_power = lens.1 .1;
                (lens_pos as u32 * lens_power) + acc
            });
            (box_pos as u32 * box_power) + acc
        });

        let sum = i64::try_from(sum).context("")?;

        Ok(sum)
    }
}
