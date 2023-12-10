use std::str::FromStr;

use advent_of_code_2023::day_8::edge::Edge;

#[test]
pub fn from_str_reads_string() {
    let actual = Edge::from_str("AAA = (BBB, CCC)").unwrap();
    let expected = Edge::new("AAA", "BBB", "CCC");

    assert_eq!(expected, actual);
}
