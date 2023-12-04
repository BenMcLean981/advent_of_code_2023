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
