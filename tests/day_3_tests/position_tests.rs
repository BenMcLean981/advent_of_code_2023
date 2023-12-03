use advent_of_code_2023::day_3::position::Position;

#[test]
pub fn is_adjacent_left_returns_true() {
    let p1 = Position::new(5, 4);
    let p2 = Position::new(5, 3);

    assert_eq!(true, p1.is_adjacent(p2));
}

#[test]
pub fn is_adjacent_right_returns_true() {
    let p1 = Position::new(5, 4);
    let p2 = Position::new(5, 5);

    assert_eq!(true, p1.is_adjacent(p2));
}

#[test]
pub fn is_adjacent_up_returns_false() {
    let p1 = Position::new(5, 4);
    let p2 = Position::new(6, 4);

    assert_eq!(false, p1.is_adjacent(p2));
}

#[test]
pub fn is_adjacent_down_returns_false() {
    let p1 = Position::new(5, 4);
    let p2 = Position::new(4, 4);

    assert_eq!(false, p1.is_adjacent(p2));
}

#[test]
pub fn is_adjacent_too_far_returns_false() {
    let p1 = Position::new(5, 4);
    let p2 = Position::new(5, 2);
    let p3 = Position::new(5, 6);

    assert_eq!(false, p1.is_adjacent(p2));
    assert_eq!(false, p1.is_adjacent(p3));
}

#[test]
pub fn is_neighboring_left_returns_true() {
    let p1 = Position::new(5, 4);
    let p2 = Position::new(5, 3);

    assert_eq!(true, p1.is_neighboring(p2));
}

#[test]
pub fn is_neighboring_right_returns_true() {
    let p1 = Position::new(5, 4);
    let p2 = Position::new(5, 5);

    assert_eq!(true, p1.is_neighboring(p2));
}

#[test]
pub fn is_neighboring_up_returns_true() {
    let p1 = Position::new(5, 4);
    let p2 = Position::new(6, 4);

    assert_eq!(true, p1.is_neighboring(p2));
}

#[test]
pub fn is_neighboring_down_returns_true() {
    let p1 = Position::new(5, 4);
    let p2 = Position::new(4, 4);

    assert_eq!(true, p1.is_neighboring(p2));
}

#[test]
pub fn is_neighboring_upper_left_returns_true() {
    let p1 = Position::new(5, 4);
    let p2 = Position::new(4, 3);

    assert_eq!(true, p1.is_neighboring(p2));
}

#[test]
pub fn is_neighboring_upper_right_returns_true() {
    let p1 = Position::new(5, 4);
    let p2 = Position::new(4, 5);

    assert_eq!(true, p1.is_neighboring(p2));
}

#[test]
pub fn is_neighboring_lower_left_returns_true() {
    let p1 = Position::new(5, 4);
    let p2 = Position::new(6, 3);

    assert_eq!(true, p1.is_neighboring(p2));
}

#[test]
pub fn is_neighboring_lower_right_returns_true() {
    let p1 = Position::new(5, 4);
    let p2 = Position::new(6, 5);

    assert_eq!(true, p1.is_neighboring(p2));
}

#[test]
pub fn is_neighboring_too_far_returns_false() {
    let p1 = Position::new(5, 4);
    let p2 = Position::new(6, 6);

    assert_eq!(false, p1.is_neighboring(p2));
}
