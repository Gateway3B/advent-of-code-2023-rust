use crate::Solvable;

use anyhow::{anyhow, Context, Result};
use std::fs::read_to_string;
use std::path::Path;

use std::ops::Not;
use std::str::FromStr;
use strum_macros::{Display, EnumIs, EnumString};

use std::collections::HashSet;

#[derive(Copy, Clone, EnumString, EnumIs, Debug, Display)]
enum Pipe {
    #[strum(serialize = "|")]
    Vertical,
    #[strum(serialize = "-")]
    Horizontal,
    #[strum(serialize = "L")]
    BottomLeft,
    #[strum(serialize = "J")]
    BottomRight,
    #[strum(serialize = "F")]
    TopLeft,
    #[strum(serialize = "7")]
    TopRight,
    #[strum(serialize = ".")]
    Ground,
    #[strum(serialize = "S")]
    Start,
}

#[derive(Copy, Clone, PartialEq, Eq, EnumIs, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

impl Pipe {
    pub fn is_top(&self) -> bool {
        self.is_top_left() || self.is_top_right()
    }

    pub fn is_bottom(&self) -> bool {
        self.is_bottom_left() || self.is_bottom_right()
    }

    pub fn angle_count(&self) -> i32 {
        if self.is_top() {
            1
        } else if self.is_bottom() {
            -1
        } else {
            0
        }
    }

    pub fn can_connect(&self, other: &Self, direction: &Direction) -> bool {
        if other.is_ground() {
            return false;
        }

        if direction.is_up() {
            return match (self, other) {
                (_, Pipe::Horizontal) => false,
                (Pipe::Horizontal, _) => false,

                (_, Pipe::BottomLeft) => false,
                (_, Pipe::BottomRight) => false,

                (Pipe::TopLeft, _) => false,
                (Pipe::TopRight, _) => false,

                (_, _) => true,
            };
        }
        if direction.is_right() {
            return match (self, other) {
                (_, Pipe::Vertical) => false,
                (Pipe::Vertical, _) => false,

                (_, Pipe::TopLeft) => false,
                (_, Pipe::BottomLeft) => false,

                (Pipe::TopRight, _) => false,
                (Pipe::BottomRight, _) => false,

                (_, _) => true,
            };
        }
        if direction.is_down() {
            return match (self, other) {
                (_, Pipe::Horizontal) => false,
                (Pipe::Horizontal, _) => false,

                (_, Pipe::TopLeft) => false,
                (_, Pipe::TopRight) => false,

                (Pipe::BottomLeft, _) => false,
                (Pipe::BottomRight, _) => false,

                (_, _) => true,
            };
        }
        if direction.is_left() {
            return match (self, other) {
                (_, Pipe::Vertical) => false,
                (Pipe::Vertical, _) => false,

                (_, Pipe::TopRight) => false,
                (_, Pipe::BottomRight) => false,

                (Pipe::TopLeft, _) => false,
                (Pipe::BottomLeft, _) => false,

                (_, _) => true,
            };
        }

        false
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Coords {
    row: usize,
    col: usize,
}

#[derive(EnumIs, Debug)]
enum PathPosition {
    OutsidePath,
    OnPathPerp(Transition),
    OnPathAngled((Transition, Option<i32>)),
    InPath,
}

#[derive(Copy, Clone, EnumIs, Debug)]
enum Transition {
    Entered,
    Exited,
}

impl Not for Transition {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Transition::Entered => Transition::Exited,
            Transition::Exited => Transition::Entered,
        }
    }
}

#[derive(Debug)]
struct Map {
    pipes: Vec<Vec<Pipe>>,
    rows_count: usize,
    columns_count: usize,
    path_coords: HashSet<Coords>,
    inside_coords: HashSet<Coords>,
}

impl Map {
    pub fn neighbors(&self, coords: Coords, origin: Option<Direction>) -> Vec<(Coords, Direction)> {
        let mut neighbors = Vec::new();

        let top_row = coords.row.checked_sub(1);
        let right_col = coords.col + 1;
        let bottom_row = coords.row + 1;
        let left_col = coords.col.checked_sub(1);

        if let Some(top_row) = top_row {
            let top = Coords {
                row: top_row,
                col: coords.col,
            };
            if !origin.is_some_and(|origin| origin.is_up()) {
                neighbors.push((top, Direction::Up));
            }
        }
        if right_col <= self.columns_count {
            let right = Coords {
                row: coords.row,
                col: right_col,
            };
            if !origin.is_some_and(|origin| origin.is_right()) {
                neighbors.push((right, Direction::Right));
            }
        }
        if bottom_row <= self.rows_count {
            let bottom = Coords {
                row: bottom_row,
                col: coords.col,
            };
            if !origin.is_some_and(|origin| origin.is_down()) {
                neighbors.push((bottom, Direction::Down));
            }
        }
        if let Some(left_col) = left_col {
            let left = Coords {
                row: coords.row,
                col: left_col,
            };
            if !origin.is_some_and(|origin| origin.is_left()) {
                neighbors.push((left, Direction::Left));
            }
        }

        neighbors
    }

    pub fn get_pipe(&self, coords: Coords) -> Option<Pipe> {
        self.pipes
            .get(coords.row)?
            .get(coords.col)
            .and_then(|coords| Some(coords.clone()))
    }

    pub fn find_connection(
        &self,
        pipe: Pipe,
        coords: Coords,
        origin: Option<Direction>,
    ) -> Option<(Coords, Direction)> {
        let neighbors = self.neighbors(coords, origin);

        neighbors
            .into_iter()
            .find(|(coords, direction)| {
                self.get_pipe(coords.clone())
                    .and_then(|other_pipe| pipe.can_connect(&other_pipe, direction).then(|| ()))
                    .is_some()
            })
            .and_then(|(coords, direction)| Some((coords, !direction)))
    }
}

pub struct Day10 {}

impl Solvable for Day10 {
    fn get_day() -> u32 {
        10
    }

    fn solve_part_one(debug: bool) -> Result<i64> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let mut start = Coords { row: 0, col: 0 };

        let pipes = read_to_string(path)?
            .lines()
            .enumerate()
            .map(|(row, line)| {
                (|| {
                    line.chars()
                        .enumerate()
                        .map(|(col, char)| {
                            (|| {
                                let pipe = Pipe::from_str(char.to_string().as_str())
                                    .map_err(anyhow::Error::msg)?;
                                if pipe.is_start() {
                                    start = Coords { row, col };
                                }
                                Ok(pipe)
                            })()
                        })
                        .collect::<Result<Vec<Pipe>>>()
                })()
            })
            .collect::<Result<Vec<Vec<Pipe>>>>()?;

        let rows_count = pipes.len();
        let columns_count = pipes.get(0).context("No rows")?.len();

        let map = Map {
            pipes,
            rows_count,
            columns_count,
            path_coords: HashSet::new(),
            inside_coords: HashSet::new(),
        };

        if debug {
            println!("{:#?}", start);
            println!("{:#?}", map);
        }

        let max_path = map.rows_count * map.columns_count;

        let mut pipe = Pipe::Start;
        let mut coords = start.clone();
        let mut origin = None;
        let mut path_length = 0;
        loop {
            let connection = map
                .find_connection(pipe, coords, origin)
                .context("Could not find next connection.")?;
            coords = connection.0;
            origin = Some(connection.1);
            pipe = map.get_pipe(coords).context("Can't get pipe")?;

            path_length += 1;

            if debug {
                println!(
                    "{}\tCoords: {:?}\n\tOrigin: {:?}\n\tPipe: {:?}",
                    path_length, coords, origin, pipe
                );
            }

            if path_length >= max_path {
                return Err(anyhow!("Took too many pipes"));
            }

            if pipe.is_start() {
                break;
            }
        }

        let steps_to_furthest_pipe = path_length / 2;
        
        let steps_to_furthest_pipe = i64::try_from(steps_to_furthest_pipe)?;
        Ok(steps_to_furthest_pipe)
    }

    fn solve_part_two(debug: bool) -> Result<i64> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let mut start = Coords { row: 0, col: 0 };

        let pipes = read_to_string(path)?
            .lines()
            .enumerate()
            .map(|(row, line)| {
                (|| {
                    line.chars()
                        .enumerate()
                        .map(|(col, char)| {
                            (|| {
                                let pipe = Pipe::from_str(char.to_string().as_str())
                                    .map_err(anyhow::Error::msg)?;
                                if pipe.is_start() {
                                    start = Coords { row, col };
                                }
                                Ok(pipe)
                            })()
                        })
                        .collect::<Result<Vec<Pipe>>>()
                })()
            })
            .collect::<Result<Vec<Vec<Pipe>>>>()?;

        let rows_count = pipes.len();
        let columns_count = pipes.get(0).context("No rows")?.len();

        let mut map = Map {
            pipes,
            rows_count,
            columns_count,
            path_coords: HashSet::new(),
            inside_coords: HashSet::new(),
        };

        if debug {
            println!("{:#?}", start);
            println!("{:#?}", map);
        }

        let max_path = map.rows_count * map.columns_count;

        let mut pipe = Pipe::Start;
        let mut coords = start.clone();
        let mut origin = None;
        let mut path_length = 0;
        loop {
            let connection = map
                .find_connection(pipe, coords, origin)
                .context("Could not find next connection.")?;
            coords = connection.0;
            origin = Some(connection.1);
            pipe = map.get_pipe(coords).context("Can't get pipe")?;

            map.path_coords.insert(coords.clone());

            path_length += 1;

            if debug {
                println!(
                    "{}\tCoords: {:?}\n\tOrigin: {:?}\n\tPipe: {:?}",
                    path_length, coords, origin, pipe
                );
            }

            if path_length >= max_path {
                return Err(anyhow!("Took too many pipes"));
            }

            if pipe.is_start() {
                break;
            }
        }

        if debug {
            println!("{:#?}", map.path_coords);
        }

        let interior_tiles = map
            .pipes
            .iter()
            .enumerate()
            .fold(0, |interior_tiles, (row, cols)| {
                interior_tiles
                    + cols
                        .iter()
                        .enumerate()
                        .fold(
                            (0, PathPosition::OutsidePath),
                            |(prev_interior_tiles, prev_path_position), (col, pipe)| {
                                let coords = Coords { row, col };
                                let is_path_pipe = map.path_coords.contains(&coords);

                                if debug {
                                    println!("\t{:?}", prev_path_position);
                                    println!("{:?}", coords);
                                }

                                let curr_path_position = match (&prev_path_position, is_path_pipe) {
                                    (PathPosition::OutsidePath, true) => {
                                        if pipe.is_vertical() {
                                            PathPosition::OnPathPerp(Transition::Entered)
                                        } else {
                                            PathPosition::OnPathAngled((
                                                Transition::Entered,
                                                Some(pipe.angle_count()),
                                            ))
                                        }
                                    }
                                    (PathPosition::InPath, true) => {
                                        if pipe.is_vertical() {
                                            PathPosition::OnPathPerp(Transition::Exited)
                                        } else {
                                            PathPosition::OnPathAngled((
                                                Transition::Exited,
                                                Some(pipe.angle_count()),
                                            ))
                                        }
                                    }
                                    (
                                        PathPosition::OnPathAngled((transition, angle_count)),
                                        true,
                                    ) => {
                                        if pipe.is_vertical() {
                                            PathPosition::OnPathPerp(!*transition)
                                        } else {
                                            if pipe.is_horizontal() {
                                                prev_path_position
                                            } else if let Some(angle_count) = angle_count {
                                                if angle_count + pipe.angle_count() == 0 {
                                                    PathPosition::OnPathAngled((
                                                        *transition,
                                                        None,
                                                    ))
                                                } else {
                                                    PathPosition::OnPathAngled((
                                                        !*transition,
                                                        None,
                                                    ))
                                                }
                                            } else {
                                                PathPosition::OnPathAngled((
                                                    !*transition,
                                                    Some(pipe.angle_count()),
                                                ))
                                            }
                                        }
                                    }
                                    (PathPosition::OnPathAngled((transition, _)), false) => {
                                        if transition.is_entered() {
                                            map.inside_coords.insert(coords);
                                            PathPosition::InPath
                                        } else {
                                            PathPosition::OutsidePath
                                        }
                                    }
                                    (PathPosition::OnPathPerp(transition), true) => {
                                        if pipe.is_vertical() {
                                            PathPosition::OnPathPerp(!*transition)
                                        } else {
                                            PathPosition::OnPathAngled((
                                                !*transition,
                                                Some(pipe.angle_count()),
                                            ))
                                        }
                                    }
                                    (PathPosition::OnPathPerp(transition), false) => {
                                        if transition.is_entered() {
                                            map.inside_coords.insert(coords);
                                            PathPosition::InPath
                                        } else {
                                            PathPosition::OutsidePath
                                        }
                                    }
                                    (PathPosition::InPath, false) => {
                                        PathPosition::InPath
                                    }
                                    (_, _) => prev_path_position,
                                };

                                if curr_path_position.is_in_path() {
                                    map.inside_coords.insert(coords);
                                    (prev_interior_tiles + 1, curr_path_position)
                                } else {
                                    (prev_interior_tiles, curr_path_position)
                                }
                            },
                        )
                        .0
            });

        if debug {
            map.pipes.iter().enumerate().for_each(|(row, cols)| {
                cols.iter().enumerate().for_each(|(col, pipe)| {
                    let char = if map.inside_coords.contains(&Coords { row, col }) {
                        String::from("I")
                    } else {
                        pipe.to_string()
                    };
                    print!("{}", char);
                });
                println!("");
            })
        }

        Ok(interior_tiles)
    }
}
