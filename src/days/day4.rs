use crate::Solvable;

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

pub struct Day4 {}

impl Solvable for Day4 {
    fn get_day() -> u32 {
        4
    }

    fn solve_part_one(debug: bool) -> Result<u32> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);
        let sum = read_to_string(path)?
            .lines()
            .try_fold(0, |total_points, line| {
                let number_sets = line
                    .split_once(": ")?
                    .1
                    .split(" | ")
                    .map(|numbers| {
                        numbers
                            .split_whitespace()
                            .map(|number| number.parse::<u32>().unwrap_or_default())
                            .collect()
                    })
                    .collect::<Vec<Vec<u32>>>();

                let winning_numbers = &number_sets[0];
                let your_numbers = &number_sets[1];

                let wins = winning_numbers.iter().fold(0, |wins, winning_number| {
                    if your_numbers.contains(winning_number) {
                        return wins + 1;
                    }
                    wins
                });

                let points = if wins > 0 { 2u32.pow(wins - 1) } else { 0 };

                if debug {
                    println!("{}", line);
                    println!("Wins - {}; Points - {}", wins, points);
                }

                Some(total_points + points)
            })
            .context("Error parsing input.")?;

        Ok(sum)
    }

    fn solve_part_two(debug: bool) -> Result<u32> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let mut cards: HashMap<usize, u32> = HashMap::new();

        read_to_string(path)?
            .lines()
            .enumerate()
            .try_for_each(|(index, line)| {
                let index = index + 1;
                let card_count = if let Some(card) = cards.get_mut(&index) {
                    *card += 1;
                    card.to_owned()
                } else {
                    cards.insert(index, 1);
                    1
                };

                let number_sets = line
                    .split_once(": ")?
                    .1
                    .split(" | ")
                    .map(|numbers| {
                        numbers
                            .split_whitespace()
                            .map(|number| number.parse::<u32>().unwrap_or_default())
                            .collect()
                    })
                    .collect::<Vec<Vec<u32>>>();

                let winning_numbers = &number_sets[0];
                let your_numbers = &number_sets[1];

                let wins = winning_numbers.iter().fold(0, |wins, winning_number| {
                    if your_numbers.contains(winning_number) {
                        return wins + 1;
                    }
                    wins
                });

                (index + 1..index + wins + 1).into_iter().for_each(|index| {
                    if let Some(card) = cards.get_mut(&index) {
                        *card += card_count;
                    } else {
                        cards.insert(index, card_count);
                    }
                });

                if debug {
                    println!("Line: {}", line);
                    println!("Cards: {:?}\n", &cards);
                }

                Some(())
            })
            .context("Error parsing input.")?;

        let sum = cards.iter().map(|(_, val)| val).sum();
        Ok(sum)
    }
}
