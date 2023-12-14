use std::{cmp, str::FromStr};

use crate::utils::file_utils::read_lines;

use super::{
    sequence::Sequence,
    spring_row::{count_possible_rows, get_possible_rows, SpringRow},
};

pub fn solve() {
    let filename = "src/day_12/input.txt";

    let lines = read_lines(filename);

    let sum: usize = lines.iter().map(|l| get_num_possible(l)).sum();
    let unfolded: usize =
        lines.iter().map(|l| get_num_possible_unfolded(l)).sum();

    println!("Day 12");
    println!("The number of possible spring arrangements is {sum}.");
    println!(
        "The number of unfolded possible spring arrangements is {unfolded}."
    );
}

fn get_num_possible(line: &str) -> usize {
    let split = line
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let spring_row = SpringRow::from_str(split[0]).unwrap();
    let sequence = Sequence::from_str(split[1]).unwrap();

    return count_possible_rows(&spring_row, &sequence);
}

static mut COUNT: usize = 0;

fn get_num_possible_unfolded(line: &str) -> usize {
    let split = line
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let spring_row = SpringRow::from_str(split[0]).unwrap();
    let sequence = Sequence::from_str(split[1]).unwrap();

    let appended = get_appended_answer(&spring_row, &sequence);
    let prepended = get_prepended_answer(&spring_row, &sequence);

    unsafe {
        COUNT += 1;
        println!("{COUNT}: {line}");
    };

    if appended.is_some() && prepended.is_some() {
        return cmp::max(prepended.unwrap(), appended.unwrap());
    } else if appended.is_some() {
        return appended.unwrap();
    } else if prepended.is_some() {
        return prepended.unwrap();
    } else {
        let merged = SpringRow::merge(vec![
            &spring_row.append(),
            &spring_row.append(),
            &spring_row.append(),
            &spring_row.append(),
            &spring_row,
        ]);

        return count_possible_rows(&merged, &sequence);
    }
}

fn get_appended_answer(row: &SpringRow, sequence: &Sequence) -> Option<usize> {
    let normal = get_possible_rows(row, &sequence);
    let appended = get_possible_rows(&row.append(), &sequence);

    let sequence = sequence.expand(5);

    for n in normal.clone() {
        for a in appended.clone() {
            let merged = SpringRow::merge(vec![&a, &a, &a, &a, &n]);

            if merged.get_sequence() != sequence {
                return None;
            }
        }
    }

    let result = normal.len() * appended.len().pow(4);

    return Some(result);
}

fn get_prepended_answer(row: &SpringRow, sequence: &Sequence) -> Option<usize> {
    let normal = get_possible_rows(row, &sequence);
    let prepended = get_possible_rows(&row.prepend(), &sequence);

    let sequence = sequence.expand(5);

    for n in normal.clone() {
        for p in prepended.clone() {
            let merged = SpringRow::merge(vec![&n, &p, &p, &p, &p]);

            if merged.get_sequence() != sequence {
                return None;
            }
        }
    }

    let result = prepended.len().pow(4) * normal.len();

    return Some(result);
}
