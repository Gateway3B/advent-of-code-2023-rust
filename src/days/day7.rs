use crate::Solvable;

use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use strum_macros::EnumString;

use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Hash, EnumString, Debug)]
enum Card {
    #[strum(serialize = "A")]
    Ace,
    #[strum(serialize = "K")]
    King,
    #[strum(serialize = "Q")]
    Queen,
    #[strum(serialize = "J")]
    Jack,
    #[strum(serialize = "T")]
    Ten,
    #[strum(serialize = "9")]
    Nine,
    #[strum(serialize = "8")]
    Eight,
    #[strum(serialize = "7")]
    Seven,
    #[strum(serialize = "6")]
    Six,
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "4")]
    Four,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "2")]
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    hand: Vec<Card>,
    bid: usize,
    hand_type: Option<HandType>,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ordering = self.hand_type.partial_cmp(&other.hand_type);

        if let Some(Ordering::Equal) = ordering {
            self.hand
                .iter()
                .zip(other.hand.iter())
                .find_map(|(self_card, other_card)| {
                    let ordering = self_card.partial_cmp(&other_card);
                    if let Some(Ordering::Equal) = ordering {
                        None
                    } else {
                        ordering
                    }
                })
        } else {
            ordering
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn hand_type(self: &Self) -> HandType {
        let mut card_counts: HashMap<Card, u32> = HashMap::new();

        self.hand.iter().for_each(|card| {
            if card_counts.contains_key(card) {
                let prev_val = card_counts.get(&card).unwrap_or(&0);
                card_counts.insert(card.clone(), prev_val + 1);
            } else {
                card_counts.insert(card.clone(), 1);
            }
        });

        let mut card_counts = card_counts.into_values().collect::<Vec<u32>>();
        card_counts.sort();
        card_counts.reverse();

        match card_counts.len() {
            1 => HandType::FiveOfAKind,
            2 if card_counts[0] == 4 => HandType::FourOfAKind,
            2 => HandType::FullHouse,
            3 if card_counts[0] == 3 => HandType::ThreeOfAKind,
            3 => HandType::TwoPair,
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Hash, EnumString, Debug)]
enum CardJokers {
    #[strum(serialize = "A")]
    Ace,
    #[strum(serialize = "K")]
    King,
    #[strum(serialize = "Q")]
    Queen,
    #[strum(serialize = "T")]
    Ten,
    #[strum(serialize = "9")]
    Nine,
    #[strum(serialize = "8")]
    Eight,
    #[strum(serialize = "7")]
    Seven,
    #[strum(serialize = "6")]
    Six,
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "4")]
    Four,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "J")]
    Joker,
}

#[derive(PartialEq, Eq, Debug)]
pub struct HandJokers {
    hand: Vec<CardJokers>,
    bid: usize,
    hand_type: Option<HandType>,
}

impl PartialOrd for HandJokers {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ordering = self.hand_type.partial_cmp(&other.hand_type);

        if let Some(Ordering::Equal) = ordering {
            self.hand
                .iter()
                .zip(other.hand.iter())
                .find_map(|(self_card, other_card)| {
                    let ordering = self_card.partial_cmp(&other_card);
                    if let Some(Ordering::Equal) = ordering {
                        None
                    } else {
                        ordering
                    }
                })
        } else {
            ordering
        }
    }
}

impl Ord for HandJokers {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl HandJokers {
    fn hand_type(self: &Self) -> HandType {
        let mut card_counts: HashMap<CardJokers, u32> = HashMap::new();

        self.hand.iter().for_each(|card| {
            if card_counts.contains_key(card) {
                let prev_val = card_counts.get(&card).unwrap_or(&0);
                card_counts.insert(card.clone(), prev_val + 1);
            } else {
                card_counts.insert(card.clone(), 1);
            }
        });

        let jokers_count = if card_counts.len() != 1 || !card_counts.contains_key(&CardJokers::Joker)
        {
            card_counts
                .remove_entry(&CardJokers::Joker)
                .and_then(|entry| Some(entry.1))
                .unwrap_or_default()
        } else {
            0
        };

        let mut card_counts = card_counts.into_values().collect::<Vec<u32>>();
        card_counts.sort();
        card_counts.reverse();

        card_counts[0] += jokers_count;

        match card_counts.len() {
            1 => HandType::FiveOfAKind,
            2 if card_counts[0] == 4 => HandType::FourOfAKind,
            2 => HandType::FullHouse,
            3 if card_counts[0] == 3 => HandType::ThreeOfAKind,
            3 => HandType::TwoPair,
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!(),
        }
    }
}

pub struct Day7 {}

impl Solvable for Day7 {
    fn get_day() -> u32 {
        7
    }

    fn solve_part_one(debug: bool) -> Result<u32> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let mut hands = read_to_string(path)?
            .lines()
            .map(|line| {
                (|| {
                    let mut line_tokens = line.split_whitespace();
                    let hand = line_tokens.next().context("")?;
                    let hand = hand
                        .chars()
                        .map(|card| {
                            Card::from_str(card.to_string().as_str()).map_err(anyhow::Error::msg)
                        })
                        .collect::<Result<Vec<Card>>>()?;

                    let bid = line_tokens.next().context("")?.parse()?;

                    let mut hand = Hand {
                        hand,
                        bid,
                        hand_type: None,
                    };
                    hand.hand_type = Some(hand.hand_type());

                    Ok(hand)
                })()
            })
            .collect::<Result<Vec<Hand>>>()?;

        if debug {
            println!("{:#?}", &hands);
        }

        hands.sort();

        if debug {
            println!("{:#?}", &hands);
        }

        let total_winnings = hands
            .iter()
            .rev()
            .enumerate()
            .fold(0, |total_winnings, (index, hand)| {
                total_winnings + ((index + 1) * hand.bid)
            });

        let total_winnings = u32::try_from(total_winnings)?;

        Ok(total_winnings)
    }

    fn solve_part_two(debug: bool) -> Result<u32> {
        let path = format!("src/inputs/day{}.txt", Self::get_day());
        let path = Path::new(&path);

        let mut hands = read_to_string(path)?
            .lines()
            .map(|line| {
                (|| {
                    let mut line_tokens = line.split_whitespace();
                    let hand = line_tokens.next().context("")?;
                    let hand = hand
                        .chars()
                        .map(|card| {
                            CardJokers::from_str(card.to_string().as_str())
                                .map_err(anyhow::Error::msg)
                        })
                        .collect::<Result<Vec<CardJokers>>>()?;

                    let bid = line_tokens.next().context("")?.parse()?;

                    let mut hand = HandJokers {
                        hand,
                        bid,
                        hand_type: None,
                    };
                    hand.hand_type = Some(hand.hand_type());

                    Ok(hand)
                })()
            })
            .collect::<Result<Vec<HandJokers>>>()?;

        if debug {
            println!("{:#?}", &hands);
        }

        hands.sort();

        if debug {
            println!("{:#?}", &hands);
        }

        let total_winnings = hands
            .iter()
            .rev()
            .enumerate()
            .fold(0, |total_winnings, (index, hand)| {
                total_winnings + ((index + 1) * hand.bid)
            });

        let total_winnings = u32::try_from(total_winnings)?;

        Ok(total_winnings)
    }
}
