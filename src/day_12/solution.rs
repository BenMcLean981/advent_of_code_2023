use std::{collections::HashMap, str::FromStr};

use crate::utils::file_utils::read_lines;

use super::{
    sequence::Sequence,
    spring_row::{count_possible_rows, SpringRow, SpringType},
};

pub fn solve() {
    let filename = "src/day_12/input.txt";

    let lines = read_lines(filename);

    let mut memo = HashMap::<(SpringRow, Sequence), usize>::new();

    let sum: usize = lines.iter().map(|l| get_num_possible(l, &mut memo)).sum();
    // let unfolded: usize = lines
    //     .iter()
    //     .map(|l| get_num_possible_unfolded(l, &mut memo))
    //     .sum();

    println!("Day 12");
    println!("The number of possible spring arrangements is {sum}.");
    println!("Part 2 disabled for performance. I used dynamic programming but it is still slow.");

    // println!(
    //     "The number of unfolded possible spring arrangements is {unfolded}."
    // );
}

fn get_num_possible(
    line: &str,
    memo: &mut HashMap<(SpringRow, Sequence), usize>,
) -> usize {
    let split = line
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let spring_row = SpringRow::from_str(split[0]).unwrap();
    let sequence = Sequence::from_str(split[1]).unwrap();

    return count_possible_rows(&spring_row, &sequence, memo);
}

fn get_num_possible_unfolded(
    line: &str,
    memo: &mut HashMap<(SpringRow, Sequence), usize>,
) -> usize {
    let split = line
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let row = SpringRow::from_str(split[0]).unwrap();
    let sequence = Sequence::from_str(split[1]).unwrap().expand(5);

    let unknown = SpringRow::new(vec![SpringType::Unknown]);

    let row = SpringRow::merge(vec![
        &row, &unknown, &row, &unknown, &row, &unknown, &row, &unknown, &row,
    ]);

    return count_possible_rows(&row, &sequence, memo);
}
