use std::str::FromStr;

use advent_of_code_2023::day_12::{
    sequence::Sequence,
    spring_row::{SpringRow, SpringType},
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
pub fn get_sequence_gets_empty() {
    let row = SpringRow::from_str("???.###").unwrap();

    let actual = row.get_sequence();
    let expected = Sequence::new(vec![]);

    assert_eq!(expected, actual);
}

#[test]
pub fn get_sequence_gets_all_known() {
    let row = SpringRow::from_str("#.#.###.???.###").unwrap();

    let actual = row.get_sequence();
    let expected = Sequence::new(vec![1, 1, 3]);

    assert_eq!(expected, actual);
}

#[test]
pub fn get_sequence_stops_at_unknown() {
    let row = SpringRow::from_str("#.#.###???.###").unwrap();

    let actual = row.get_sequence();
    let expected = Sequence::new(vec![1, 1]);

    assert_eq!(expected, actual);
}
