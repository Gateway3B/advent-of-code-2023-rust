mod days;
use days::*;

use anyhow::Result;

#[derive(Clone, Copy, PartialEq)]
pub enum RunMode {
    Result,
    DebugPartOne,
    DebugPartTwo,
}

pub trait Solvable {
    fn get_day() -> u32;

    fn solve_part_one(debug: bool) -> Result<i64>;
    fn solve_part_two(debug: bool) -> Result<i64>;

    fn solve(run_mode: RunMode) -> Result<()> {
        println!("Day {}:", Self::get_day());

        if [RunMode::DebugPartOne, RunMode::Result].contains(&run_mode) {
            match Self::solve_part_one(run_mode == RunMode::DebugPartOne) {
                Ok(result) => {
                    println!("\tPart 1 - {}", result);
                }
                Err(err) => {
                    println!("Part 1 Error - {}", err);
                }
            }
        }
        if [RunMode::DebugPartTwo, RunMode::Result].contains(&run_mode) {
            match Self::solve_part_two(run_mode == RunMode::DebugPartTwo) {
                Ok(result) => {
                    println!("\tPart 2 - {}", result);
                }
                Err(err) => {
                    println!("Part 2 Error - {}", err);
                }
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    Day1::solve(RunMode::Result)?;
    Day2::solve(RunMode::Result)?;
    Day3::solve(RunMode::Result)?;
    Day4::solve(RunMode::Result)?;
    Day5::solve(RunMode::Result)?;
    Day6::solve(RunMode::Result)?;
    Day7::solve(RunMode::Result)?;
    Day8::solve(RunMode::Result)?;
    Day9::solve(RunMode::Result)?;
    Day10::solve(RunMode::Result)?;
    Day11::solve(RunMode::Result)?;
    // Day12::solve(RunMode::Result)?;
    Day13::solve(RunMode::Result)?;
    Day14::solve(RunMode::Result)?;
    Day15::solve(RunMode::Result)?;

    Ok(())
}
