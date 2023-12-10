use std::str::FromStr;

use advent_of_code_2023::day_9::history::History;

#[test]
pub fn from_str_makes_history() {
    let actual = History::from_str("1 3 6 10 15 21").unwrap();
    let expected = History::new(vec![1, 3, 6, 10, 15, 21]);

    assert_eq!(expected, actual);
}

#[test]
pub fn compute_derivative_makes_derivative() {
    let history = History::new(vec![1, 3, 6, 10, 15, 21]);

    let actual = history.compute_derivative();
    let expected = History::new(vec![2, 3, 4, 5, 6]);

    assert_eq!(expected, actual);
}

#[test]
pub fn is_zero_all_zeroes_returns_true() {
    let history = History::new(vec![0, 0, 0, 0]);

    assert_eq!(true, history.is_zero());
}

#[test]
pub fn is_zero_empty_returns_true() {
    let history = History::new(vec![]);

    assert_eq!(true, history.is_zero());
}

#[test]
pub fn is_zero_one_non_zero_returns_false() {
    let history = History::new(vec![0, 1, 0, 0]);

    assert_eq!(false, history.is_zero());
}

#[test]
pub fn get_next_zeros_returns_zero() {
    let history = History::new(vec![0, 0, 0]);

    assert_eq!(0, history.get_next());
}

#[test]
pub fn zero_derivative_returns_last() {
    let history = History::new(vec![5, 5, 5]);

    assert_eq!(5, history.get_next());
}

#[test]
pub fn non_zero_derivative_computes_recursively() {
    let history = History::new(vec![10, 13, 16, 21, 30, 45]);

    assert_eq!(68, history.get_next());
}
