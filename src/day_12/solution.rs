use std::str::FromStr;

use crate::utils::file_utils::read_lines;

use super::{
    sequence::Sequence,
    spring_row::{count_possible_rows, SpringRow},
};

pub fn solve() {
    let filename = "src/day_12/input.txt";

    let lines = read_lines(filename);

    let num_possible = lines
        .iter()
        .map(|l| get_num_possible(l))
        .collect::<Vec<usize>>();

    let sum: usize = num_possible.iter().sum();

    println!("Day 12");
    println!("The number of possible spring arrangements is {sum}.");
}

fn get_num_possible(line: &str) -> usize {
    let split = line
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let spring_row = SpringRow::from_str(split[0]).unwrap();
    let sequence = Sequence::from_str(split[1]).unwrap();

    return count_possible_rows(spring_row, &sequence);
}
