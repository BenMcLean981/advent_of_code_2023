use advent_of_code_2023::day_5::range::Range;

#[test]
#[should_panic]
pub fn new_zero_length_panics() {
    Range::new(5, 0);
}

#[test]
pub fn new_non_zero_length_returns_range() {
    let range = Range::new(5, 2);

    assert_eq!(5, range.lower);
    assert_eq!(6, range.upper);
}

#[test]
pub fn contains_too_small_returns_false() {
    let range = Range::new(5, 2);

    assert_eq!(false, range.contains(4));
}

#[test]
pub fn contains_too_large_returns_false() {
    let range = Range::new(5, 2);

    assert_eq!(false, range.contains(7));
}

#[test]
pub fn contains_lower_returns_true() {
    let range = Range::new(5, 3);

    assert_eq!(true, range.contains(5));
}

#[test]
pub fn contains_middle_returns_true() {
    let range = Range::new(5, 3);

    assert_eq!(true, range.contains(6));
}

#[test]
pub fn contains_upper_returns_true() {
    let range = Range::new(5, 3);

    assert_eq!(true, range.contains(7));
}
