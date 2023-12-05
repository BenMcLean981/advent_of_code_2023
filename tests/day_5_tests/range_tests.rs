use advent_of_code_2023::day_5::range::Range;

#[test]
pub fn new_zero_length_returns_range() {
    let range = Range::new(5, 0);

    assert_eq!(5, range.lower);
    assert_eq!(5, range.upper);
}

#[test]
pub fn new_non_zero_length_returns_range() {
    let range = Range::new(5, 2);

    assert_eq!(5, range.lower);
    assert_eq!(7, range.upper);
}

#[test]
pub fn contains_too_small_returns_false() {
    let range = Range::new(5, 2);

    assert_eq!(false, range.contains(4));
}

#[test]
pub fn contains_too_large_returns_false() {
    let range = Range::new(5, 2);

    assert_eq!(false, range.contains(8));
}

#[test]
pub fn contains_lower_returns_true() {
    let range = Range::new(5, 2);

    assert_eq!(true, range.contains(5));
}

#[test]
pub fn contains_middle_returns_true() {
    let range = Range::new(5, 2);

    assert_eq!(true, range.contains(6));
}

#[test]
pub fn contains_upper_returns_true() {
    let range = Range::new(5, 2);

    assert_eq!(true, range.contains(7));
}
