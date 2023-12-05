use advent_of_code_2023::day_5::range::Range;

use crate::day_5_tests::utils::have_same_items;

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

#[test]
pub fn intersection_no_overlap_returns_none() {
    let r1 = Range::new(5, 3);
    let r2 = Range::new(8, 3);

    assert_eq!(None, Range::intersection(r1, r2))
}

#[test]
pub fn intersection_equal_returns_equal() {
    let r1 = Range::new(5, 3);
    let r2 = r1.clone();

    assert_eq!(r1.clone(), Range::intersection(r1, r2).unwrap())
}

#[test]
pub fn intersection_subset_returns_subset() {
    let r1 = Range::new(5, 5);
    let r2 = Range::new(7, 2);

    assert_eq!(r2, Range::intersection(r1, r2).unwrap());
}

#[test]
pub fn intersection_superset_returns_subset() {
    let r1 = Range::new(7, 2);
    let r2 = Range::new(5, 5);

    assert_eq!(r1, Range::intersection(r1, r2).unwrap());
}

#[test]
pub fn intersection_overlap_end_returns_overlap() {
    let r1 = Range::new(5, 5);
    let r2 = Range::new(7, 5);

    assert_eq!(Range::new(7, 3), Range::intersection(r1, r2).unwrap());
}

#[test]
pub fn intersection_overlap_start_returns_overlap() {
    let r1 = Range::new(7, 5);
    let r2 = Range::new(5, 5);

    assert_eq!(Range::new(7, 3), Range::intersection(r1, r2).unwrap());
}

#[test]
pub fn intersection_touching_returns_overlap() {
    let r1 = Range::new(7, 5);
    let r2 = Range::new(11, 5);

    assert_eq!(Range::new(11, 1), Range::intersection(r1, r2).unwrap());
}

#[test]
pub fn difference_no_intersection_returns_original() {
    let base = Range::new(0, 10);
    let sub = Range::new(12, 3);

    let actual = Range::difference(base, sub);
    let expected = vec![base];

    assert_eq!(true, have_same_items(expected, actual));
}

#[test]
pub fn difference_intersects_upper_returns_lower() {
    let base = Range::new(0, 10);
    let sub = Range::new(8, 4);

    let actual = Range::difference(base, sub);
    let expected = vec![Range::new(0, 8)];

    assert_eq!(true, have_same_items(expected, actual));
}

#[test]
pub fn difference_intersects_lower_returns_upper() {
    let base = Range::new(10, 10);
    let sub = Range::new(8, 4);

    let actual = Range::difference(base, sub);
    let expected = vec![Range::new(12, 8)];

    assert_eq!(true, have_same_items(expected, actual));
}

#[test]
pub fn difference_intersects_middle_returns_ends() {
    let base = Range::new(10, 10);
    let sub = Range::new(12, 4);

    let actual = Range::difference(base, sub);
    let expected = vec![Range::new(10, 2), Range::new(16, 4)];

    assert_eq!(true, have_same_items(expected, actual));
}
