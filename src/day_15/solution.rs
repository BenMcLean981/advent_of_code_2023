use std::str::FromStr;

use crate::{day_15::instruction::Instruction, utils::file_utils::read_lines};

pub fn solve() {
    let filename = "src/day_15/input.txt";

    let content = read_lines(filename)[0].trim().chars().collect::<String>();

    let steps = content
        .split(',')
        .map(|s| Instruction::from_str(s).unwrap())
        .collect::<Vec<Instruction>>();
    let hashes = steps.iter().map(|i| i.compute_hash()).collect::<Vec<u64>>();

    let sum: u64 = hashes.iter().sum();

    println!("Day 15");
    println!("The sum of hashes of steps is {sum}.");
}
