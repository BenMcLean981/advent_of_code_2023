use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct Card {
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl Card {
    pub fn new(winning_numbers: HashSet<u32>, numbers: HashSet<u32>) -> Self {
        return Card {
            winning_numbers,
            numbers,
        };
    }

    pub fn get_score(&self) -> u32 {
        let matches: usize = self
            .numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
            .try_into()
            .unwrap();

        if matches == 0 {
            return 0;
        } else {
            let exponent: usize = matches - 1;
            let exponent: u32 = exponent.try_into().unwrap();

            return 2_i32.pow(exponent).try_into().unwrap();
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let game = s.split(':').nth(1).unwrap().trim();

        let split: Vec<&str> = game.split('|').collect();

        let winning_str = split[0].trim();
        let num_str = split[1].trim();

        return Ok(Card::new(get_numbers(winning_str), get_numbers(num_str)));
    }
}

fn get_numbers(s: &str) -> HashSet<u32> {
    return s
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| u32::from_str(s).unwrap())
        .collect();
}
