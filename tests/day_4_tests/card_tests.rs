use std::{collections::HashSet, str::FromStr};

use advent_of_code_2023::day_4::card::Card;

#[test]
pub fn from_str_no_numbers_returns_empty() {
    let actual = Card::from_str("Card 1:|").unwrap();

    let expected = Card::new(HashSet::new(), HashSet::new());

    assert_eq!(expected, actual);
}

#[test]
pub fn from_str_one_number_returns_card() {
    let actual = Card::from_str("Card 5: 9 | 512").unwrap();

    let expected = Card::new(HashSet::from([9]), HashSet::from([512]));

    assert_eq!(expected, actual);
}

#[test]
pub fn from_str_several_number_returns_card() {
    let actual = Card::from_str("Card 4: 12 5 812 | 68 1").unwrap();

    let expected =
        Card::new(HashSet::from([12, 5, 812]), HashSet::from([68, 1]));

    assert_eq!(expected, actual);
}

#[test]
pub fn score_no_winning_numbers_returns_zero() {
    let card = Card::from_str("Card 4: 12 5 812 | 68 1").unwrap();

    assert_eq!(0, card.get_score());
}

#[test]
pub fn score_one_winning_numbers_returns_one() {
    let card = Card::from_str("Card 4: 12 5 812 | 68 5").unwrap();

    assert_eq!(1, card.get_score());
}

#[test]
pub fn score_two_winning_numbers_returns_two() {
    let card = Card::from_str("Card 4: 12 5 812 | 68 5 12").unwrap();

    assert_eq!(2, card.get_score());
}

#[test]
pub fn score_four_winning_numbers_returns_eight() {
    let card =
        Card::from_str("Card 4: 12 5 812 92 18 | 18 92 68 5 12").unwrap();

    assert_eq!(8, card.get_score());
}
