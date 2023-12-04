use std::str::FromStr;

use crate::{
    day_4::{card::Card, card_count_builder::CardCountBuilder},
    utils::file_utils::read_lines,
};

pub fn solve() {
    let filename = "src/day_4/input.txt";

    let lines = read_lines(filename);
    let cards = lines
        .iter()
        .map(|l| Card::from_str(l).unwrap())
        .collect::<Vec<Card>>();

    let scores = cards.iter().map(|c| c.get_score()).collect::<Vec<u32>>();

    let total_score: u32 = scores.iter().sum();

    let mut card_counter = CardCountBuilder::new(cards);
    card_counter.build();

    let card_count = card_counter.get_count().unwrap();

    println!("Day 4");
    println!("The total score of all cards is {total_score}.");
    println!("The final number of cards is {card_count}.");
}
