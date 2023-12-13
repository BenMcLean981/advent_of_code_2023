use std::str::FromStr;

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
pub fn get_sequence_gets_empty() {
    let row = SpringRow::from_str("???.###").unwrap();

    let actual = row.get_sequence();
    let expected = Sequence::make_clear(vec![]);

    assert_eq!(expected, actual);
}

#[test]
pub fn get_sequence_gets_all_known() {
    let row = SpringRow::from_str("#.#.###.???.###").unwrap();

    let actual = row.get_sequence();
    let expected = Sequence::make_clear(vec![1, 1, 3]);

    assert_eq!(expected, actual);
}

#[test]
pub fn get_sequence_makes_unclear() {
    let row = SpringRow::from_str("#.#.###???.###").unwrap();

    let actual = row.get_sequence();

    // next could be a # or . which would make this 3 or 4.
    let expected = Sequence::make_unclear(vec![1, 1, 3]);

    assert_eq!(expected, actual);
}

#[test]
pub fn get_sequence_done_makes_clear() {
    let row = SpringRow::from_str("#.#.###.###").unwrap();

    let actual = row.get_sequence();

    // next could be a # or . which would make this 3 or 4.
    let expected = Sequence::make_clear(vec![1, 1, 3, 3]);

    assert_eq!(expected, actual);
}

#[test]
pub fn is_done_no_unknown_returns_true() {
    let row = SpringRow::from_str("#.#.###").unwrap();

    assert!(row.is_done());
}

#[test]
pub fn is_done_unknown_returns_false() {
    let row = SpringRow::from_str("#.#.##?#").unwrap();

    assert!(!row.is_done());
}

#[test]
#[should_panic]
pub fn fill_next_is_done_panics() {
    let row = SpringRow::from_str("#.#.###").unwrap();

    row.fill_next();
}

#[test]
pub fn fill_next_gets_both_options() {
    let row = SpringRow::from_str("#.#?.###").unwrap();

    let actual = row.fill_next();
    let expected = (
        SpringRow::from_str("#.#..###").unwrap(),
        SpringRow::from_str("#.##.###").unwrap(),
    );

    assert_eq!(expected, actual);
}

#[test]
pub fn count_possible_rows_counts_rows() {
    let row = SpringRow::from_str("?###????????").unwrap();
    let sequence = Sequence::make_clear(vec![3, 2, 1]);

    assert_eq!(10, count_possible_rows(row, &sequence));
}
