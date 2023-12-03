use std::collections::HashSet;

use advent_of_code_2023::day_3::{number::Number, position::Position};

#[test]
pub fn is_neighboring_too_far_returns_false() {
    let number = Number::new(
        111,
        HashSet::<Position>::from_iter(vec![
            Position::new(5, 4),
            Position::new(5, 5),
            Position::new(5, 6),
        ]),
    );

    let p = Position::new(4, 8);

    assert_eq!(false, number.is_neighboring(p));
}

#[test]
pub fn is_neighboring_diagonal_returns_true() {
    let number = Number::new(
        111,
        HashSet::<Position>::from_iter(vec![
            Position::new(5, 4),
            Position::new(5, 5),
            Position::new(5, 6),
        ]),
    );

    let p = Position::new(4, 7);

    assert_eq!(true, number.is_neighboring(p));
}

#[test]
pub fn is_neighboring_top_returns_true() {
    let number = Number::new(
        111,
        HashSet::<Position>::from_iter(vec![
            Position::new(5, 4),
            Position::new(5, 5),
            Position::new(5, 6),
        ]),
    );

    let p = Position::new(4, 5);

    assert_eq!(true, number.is_neighboring(p));
}
