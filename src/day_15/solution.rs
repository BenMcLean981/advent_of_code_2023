use crate::utils::file_utils::read_lines;

use super::hash::hash;

pub fn solve() {
    let filename = "src/day_15/input.txt";

    let content = read_lines(filename)[0].trim().chars().collect::<String>();

    let steps = content.split(',').collect::<Vec<&str>>();
    let hashes = steps.iter().map(|s| hash(s)).collect::<Vec<u32>>();

    let sum: u32 = hashes.iter().sum();

    println!("Day 15");
    println!("The sum of hashes of steps is {sum}.");
}
