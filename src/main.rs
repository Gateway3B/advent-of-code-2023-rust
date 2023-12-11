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

    fn solve_part_one(debug: bool) -> Result<u32>;
    fn solve_part_two(debug: bool) -> Result<u32>;

    fn solve(run_mode: RunMode) -> Result<()> {
        println!("Day {}:", Self::get_day());

        if [RunMode::DebugPartOne, RunMode::Result].contains(&run_mode) {
            println!(
                "\tPart 1 - {}",
                Self::solve_part_one(run_mode == RunMode::DebugPartOne).unwrap_or_default()
            );
        }
        if [RunMode::DebugPartTwo, RunMode::Result].contains(&run_mode) {
            println!(
                "\tPart 2 - {}",
                Self::solve_part_two(run_mode == RunMode::DebugPartTwo).unwrap_or_default()
            );
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    Day1::solve(RunMode::Result)?;
    Day2::solve(RunMode::Result)?;
    Day3::solve(RunMode::Result)?;
    Day4::solve(RunMode::DebugPartOne)?;

    Ok(())
}
