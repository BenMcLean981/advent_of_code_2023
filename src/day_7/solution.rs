use std::str::FromStr;

use crate::{day_7::bidded_hand::BiddedHand, utils::file_utils::read_lines};

pub fn solve() {
    let filename = "src/day_7/input.txt";

    let lines = read_lines(filename);
    let mut bidded_hands = lines
        .iter()
        .map(|l| BiddedHand::from_str(l).unwrap())
        .collect::<Vec<BiddedHand>>();

    bidded_hands.sort();

    let total_winnings: u32 = bidded_hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index as u32 + 1) * hand.bid)
        .sum();

    println!("Day 7");
    println!("The total winnings are {total_winnings}.");
}
