use std::{collections::HashMap, str::FromStr};

use super::rank::Rank;

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    card_1: Rank,
    card_2: Rank,
    card_3: Rank,
    card_4: Rank,
    card_5: Rank,
}

impl Hand {
    pub fn new(
        card_1: Rank,
        card_2: Rank,
        card_3: Rank,
        card_4: Rank,
        card_5: Rank,
    ) -> Self {
        return Hand {
            card_1,
            card_2,
            card_3,
            card_4,
            card_5,
        };
    }

    pub fn get_type(&self) -> HandType {
        let counts = self.get_counts();

        if is_one_count_of(&counts, 5) {
            return HandType::FiveOfAKind;
        } else if is_one_count_of(&counts, 4) {
            return HandType::FourOfAKind;
        } else if is_one_count_of(&counts, 3) && is_one_count_of(&counts, 2) {
            return HandType::FullHouse;
        } else if is_one_count_of(&counts, 3) {
            return HandType::ThreeOfAKind;
        } else if is_two_counts_of(&counts, 2) {
            return HandType::TwoPair;
        } else if is_one_count_of(&counts, 2) {
            return HandType::OnePair;
        } else {
            return HandType::HighCard;
        }
    }

    fn get_counts(&self) -> HashMap<Rank, u32> {
        let mut result = HashMap::<Rank, u32>::new();

        add_card_to_count(&mut result, self.card_1);
        add_card_to_count(&mut result, self.card_2);
        add_card_to_count(&mut result, self.card_3);
        add_card_to_count(&mut result, self.card_4);
        add_card_to_count(&mut result, self.card_5);

        return result;
    }
}

fn add_card_to_count(counts: &mut HashMap<Rank, u32>, card: Rank) {
    if counts.contains_key(&card) {
        counts.insert(card, counts[&card] + 1);
    } else {
        counts.insert(card, 1);
    }
}

fn is_one_count_of(counts: &HashMap<Rank, u32>, c: u32) -> bool {
    return get_num_of_cards_with_count(counts, c) == 1;
}

fn is_two_counts_of(counts: &HashMap<Rank, u32>, c: u32) -> bool {
    return get_num_of_cards_with_count(counts, c) == 2;
}

fn get_num_of_cards_with_count(counts: &HashMap<Rank, u32>, c: u32) -> u32 {
    return counts.values().filter(|v| **v == c).count() as u32;
}

#[derive(Debug, PartialEq, Eq)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .trim()
            .chars()
            .map(|c| Rank::from(c))
            .collect::<Vec<Rank>>();

        if cards.len() != 5 {
            return Err(ParseHandError);
        }

        return Ok(Hand::new(cards[0], cards[1], cards[2], cards[3], cards[4]));
    }
}
