use std::str::FromStr;

use crate::{day_7::bidded_hand::BiddedHand, utils::file_utils::read_lines};

pub fn solve() {
    let filename = "src/day_7/input.txt";

    let lines = read_lines(filename);
    let bidded_hands = lines
        .iter()
        .map(|l| BiddedHand::from_str(l).unwrap())
        .collect::<Vec<BiddedHand>>();

    let part_1 = solve_part_1(bidded_hands.clone().as_mut());
    let part_2 = solve_part_2(bidded_hands.clone().as_mut());

    println!("Day 7");
    println!("The total winnings are {part_1}.");
    println!("The total winnings with jokers are {part_2}.");
}

fn solve_part_1(bidded_hands: &mut Vec<BiddedHand>) -> u32 {
    bidded_hands.sort();

    return bidded_hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index as u32 + 1) * hand.bid)
        .sum();
}

fn solve_part_2(bidded_hands: &mut Vec<BiddedHand>) -> u32 {
    bidded_hands.sort_by(|h1, h2| h1.hand.cmp_with_jokers(&h2.hand));

    return bidded_hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index as u32 + 1) * hand.bid)
        .sum();
}
