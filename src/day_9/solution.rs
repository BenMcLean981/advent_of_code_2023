use std::str::FromStr;

use crate::{day_9::history::History, utils::file_utils::read_lines};

pub fn solve() {
    let filename = "src/day_9/sample.txt";

    let lines = read_lines(filename);
    let histories = lines
        .iter()
        .map(|l| History::from_str(l).unwrap())
        .collect::<Vec<History>>();

    let next = get_sum_of_next_entries(&histories);
    let prev = get_sum_of_prev_entries(&histories);

    println!("Day 9");
    println!("The sum of the forward extrapolated values is {next}.");
    println!("The sum of the backward extrapolated values is {prev}.");
}

fn get_sum_of_next_entries(histories: &Vec<History>) -> i32 {
    return histories.iter().map(|h| h.get_next()).sum();
}

fn get_sum_of_prev_entries(histories: &Vec<History>) -> i32 {
    return histories.iter().map(|h| h.get_prev()).sum();
}
