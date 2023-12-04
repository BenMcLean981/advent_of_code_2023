use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    pub card_num: u32,
    pub quantity: u32,
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl Card {
    pub fn new(
        card_num: u32,
        quantity: u32,
        winning_numbers: HashSet<u32>,
        numbers: HashSet<u32>,
    ) -> Self {
        return Card {
            card_num,
            quantity,
            winning_numbers,
            numbers,
        };
    }

    pub fn get_score(&self) -> u32 {
        let matches = self.get_num_winning_numbers();

        if matches == 0 {
            return 0;
        } else {
            let exponent: u32 = matches - 1;
            let exponent: u32 = exponent.try_into().unwrap();

            return 2_i32.pow(exponent).try_into().unwrap();
        }
    }

    pub fn get_num_winning_numbers(&self) -> u32 {
        return self
            .numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
            .try_into()
            .unwrap();
    }

    pub fn add(&self, quantity: u32) -> Card {
        return Card::new(
            self.card_num,
            self.quantity + quantity,
            self.winning_numbers.clone(),
            self.numbers.clone(),
        );
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(':').collect();

        let game_num_str = split[0].trim();
        let game_num: u32 =
            game_num_str.split(' ').last().unwrap().parse().unwrap();

        let num_strs = split[1].trim();

        let split: Vec<&str> = num_strs.split('|').collect();

        let winning_str = split[0].trim();
        let num_str = split[1].trim();

        return Ok(Card::new(
            game_num,
            1,
            get_numbers(winning_str),
            get_numbers(num_str),
        ));
    }
}

fn get_numbers(s: &str) -> HashSet<u32> {
    return s
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| u32::from_str(s).unwrap())
        .collect();
}
