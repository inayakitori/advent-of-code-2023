mod splitting;

use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::ops::{Deref, Index};
use std::str::FromStr;

use thiserror::Error;
use crate::splitting::*;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let cards: Vec<Card> = input.lines().flat_map(|line|{
        line.parse::<Card>()
    }).collect();

    let mut counts= vec![1u64 ; cards.len()];

    for i in 0..counts.len() {
        let count = counts[i];
        let card_matches = cards[i].get_matches();
        for j in i+1..=counts.len().min(i+card_matches as usize) {
            counts[j] += count;
        }
    }

    let sum: u64 = counts.iter().sum();
    println!("total {sum} cards");

}


#[derive(Debug, Clone)]
struct Card{
    index: u32,
    winning_numbers: Numbers,
    our_numbers: Numbers
}

impl Card{
    fn get_matches(&self) -> u32 {
        self.our_numbers.0.iter().map(|our_number| {
            if self.winning_numbers.0.contains(our_number) {
                1
            } else {
                0
            }
        }).sum()
    }

    fn get_points(&self) -> u32 {
        return match self.get_matches() {
            0 => 0,
            i => 2u32.pow(i - 1)
        }
    }
}

impl FromStr for Card{
    type Err = ParseError;
    fn from_str(card_text: &str) -> Result<Self, Self::Err> {
        let (initial_text, all_numbers) =  card_text.split_once_safe(":")?;

        let index = initial_text
            .strip_prefix("Card").ok_or(ParseError::TextNotFoundError("Card".to_string()))?
            .chars().filter(|c| c.is_numeric()).collect::<String>()
            .parse::<u32>()?;

        let (winning_text, our_text) = all_numbers.split_once_safe("|")?;

        let winning_numbers : Numbers = winning_text.parse()?;
        let our_numbers: Numbers = our_text.parse()?;

        Ok(Card {
            index,
            winning_numbers,
            our_numbers,
        })
    }
}


impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card {:>3}:", self.index)?;
        for winning_number in &self.winning_numbers.0{
            write!(f, "{:>3}", winning_number)?;
        }
        write!(f, " |")?;
        for our_number in &self.our_numbers.0{
            write!(f, "{:>3}", our_number)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Numbers(Vec<u32>);

impl FromStr for Numbers{
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let parsing: Vec<Result<u32, ParseError>> = text.split_ascii_whitespace()
            .map(|num_txt| num_txt.parse::<u32>().map_err(|err| ParseError::ParseIntError(err)))
            .collect();
        if let Some(Err(parse_error)) = parsing.iter().cloned().find(|int_parsing| int_parsing.is_err())
        {
            Err(parse_error)
        } else {
            let numbers: Vec<u32> = parsing.iter().cloned()
                .map(|int_parsed| int_parsed.unwrap())
                .collect();
            Ok(Numbers(numbers))
        }
    }
}

impl Display for Numbers{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for num in &self.0 {
            write!(f, "{:>3}", num)?;
        }
        Ok(())
    }
}

impl Deref for Numbers{
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self
    }
}