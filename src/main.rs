mod days;
use days::*;

use anyhow::Result;

pub trait Solvable {
    fn get_day() -> u32;
    
    fn solve_part_one() -> Result<u32>;
    fn solve_part_two() -> Result<u32>;

    fn solve() -> Result<()> {
        println!("Day {}: {}, {}", Self::get_day(), Self::solve_part_one()?, Self::solve_part_two()?);
        Ok(())
    }
}

fn main() -> Result<()> {
    Day1::solve()?;
    Day2::solve()?;
    Day3::solve()?;
    
    Ok(())
}