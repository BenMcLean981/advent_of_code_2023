use std::{collections::HashMap, str::FromStr};

use advent_of_code_2023::day_12::{
    sequence::Sequence,
    spring_row::{count_possible_rows, SpringRow, SpringType},
};

#[test]
pub fn from_str_makes_row() {
    let actual = SpringRow::from_str("???.###").unwrap();
    let expected = SpringRow::new(vec![
        SpringType::Unknown,
        SpringType::Unknown,
        SpringType::Unknown,
        SpringType::Operational,
        SpringType::Damaged,
        SpringType::Damaged,
        SpringType::Damaged,
    ]);

    assert_eq!(actual, expected);
}

#[test]
pub fn force_count_possible_rows_counts_rows() {
    let mut memo = HashMap::<(SpringRow, Sequence), usize>::new();

    let row = SpringRow::from_str("?###????????").unwrap();
    let sequence = Sequence::make_clear(vec![3, 2, 1]);

    assert_eq!(10, count_possible_rows(&row, &sequence, &mut memo));
}
