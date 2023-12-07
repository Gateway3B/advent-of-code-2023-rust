use crate::Solvable;

use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
struct Coord {
    row: i32,
    col: i32,
}

#[derive(Debug)]
struct PartNumber {
    number: u32,
    digits: i32,
    coord: Coord,
}

impl PartNumber {
    pub fn adjacent_to(self: &Self, coord: &Coord) -> bool {
        (coord.row - self.coord.row).abs() <= 1 &&
        (0..self.digits).into_iter().any(|index| {
            (coord.col - self.coord.col - index).abs() <= 1
        })
    }
}

pub struct Day3 {}

impl Solvable for Day3 {
    fn get_day() -> u32 {
        3
    }

    fn solve_part_one() -> Result<u32> {
        let mut symbol_coords: Vec<Coord> = Vec::new();
        let mut part_numbers: Vec<PartNumber> = Vec::new();
    
        let path = Path::new("src/inputs/day3.txt");
        read_to_string(path)?
            .lines()
            .enumerate()
            .try_for_each(|(row, line)| {
                let mut number_str = String::new();
                let line_len = i32::try_from(line.len()).ok()?;
                let mut coord_col = 0;
                line.chars().enumerate().try_for_each(|(col, char)| {
                    let col = i32::try_from(col).ok()?;
                    let row = i32::try_from(row).ok()?;
    
                    if char.is_numeric() {
                        if number_str.len() == 0 {
                            coord_col = col;
                        }
                        number_str.push(char);
                        if col != line_len - 1 {
                            return Some(());
                        }
                    }
                    
                    if char.is_ascii_punctuation() {
                        if char != '.' {
                            symbol_coords.push(Coord { row, col });
                        }
                    }
    
                    if number_str.len() > 0 {
                        let number = number_str.parse().ok()?;
                        let digits = i32::try_from(number_str.len()).ok()?;
                        let coord = Coord { row, col: coord_col };
                        part_numbers.push(PartNumber {
                            number,
                            digits,
                            coord,
                        });
                        number_str = String::new();
                    }
    
                    Some(())
                })?;
                
                Some(())
            }).context("")?;
    
        // println!("{:#?}", symbol_coords);
        // println!("{:#?}", part_numbers);
        
        let sum = symbol_coords.iter().fold(0u32, |sum, symbol_coord| {
            let mut new_sum = sum;
    
            part_numbers.retain(|part_number| {
                if part_number.adjacent_to(symbol_coord) {
                    new_sum += part_number.number;
                    return false;
                }
    
                part_number.coord.row >= symbol_coord.row - 1
            });
    
            new_sum
        });
    
        Ok(sum)
    }
    
    
    fn solve_part_two() -> Result<u32> {
        let mut symbol_coords: Vec<Coord> = Vec::new();
        let mut part_numbers: Vec<PartNumber> = Vec::new();
    
        let path = Path::new("src/inputs/day3.txt");
        read_to_string(path)?
            .lines()
            .enumerate()
            .try_for_each(|(row, line)| {
                let mut number_str = String::new();
                let line_len = i32::try_from(line.len()).ok()?;
                let mut coord_col = 0;
                line.chars().enumerate().try_for_each(|(col, char)| {
                    let col = i32::try_from(col).ok()?;
                    let row = i32::try_from(row).ok()?;
    
                    if char.is_numeric() {
                        if number_str.len() == 0 {
                            coord_col = col;
                        }
                        number_str.push(char);
                        if col != line_len - 1 {
                            return Some(());
                        }
                    }
    
                    if char.is_ascii_punctuation() {
                        if char != '.' {
                            symbol_coords.push(Coord { row, col });
                        }
                    }
    
                    if number_str.len() > 0 {
                        let number = number_str.parse().ok()?;
                        let digits = i32::try_from(number_str.len()).ok()?;
                        let coord = Coord { row, col: coord_col };
                        part_numbers.push(PartNumber {
                            number,
                            digits,
                            coord,
                        });
                        number_str = String::new();
                    }
    
                    Some(())
                })?;
    
                Some(())
            }).context("")?;
    
        // println!("{:#?}", symbol_coords);
        // println!("{:#?}", part_numbers);
    
        let sum = symbol_coords.iter().fold(0u32, |sum, symbol_coord| {
            let mut new_sum = sum;
    
            part_numbers.retain(|part_number| {
                if part_number.adjacent_to(symbol_coord) {
                    new_sum += part_number.number;
                    return false;
                }
    
                part_number.coord.row >= symbol_coord.row - 1
            });
    
            new_sum
        });
    
        Ok(sum)
    }
}
