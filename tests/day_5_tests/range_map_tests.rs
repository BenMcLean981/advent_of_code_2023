use advent_of_code_2023::day_5::{range::Range, range_map::RangeMap};

#[test]
#[should_panic]
pub fn new_different_sizes_panics() {
    RangeMap::new(Range::new(5, 2), Range::new(10, 3));
}

#[test]
pub fn new_same_sizes_does_not_panic() {
    let size = 2;
    RangeMap::new(Range::new(5, size), Range::new(10, size));
}

#[test]
pub fn map_in_range_maps() {
    let size = 3;
    let map = RangeMap::new(Range::new(5, size), Range::new(10, size));

    assert_eq!(10, map.map(5));
    assert_eq!(12, map.map(7));
}

#[test]
pub fn map_out_of_range_does_not_map() {
    let size = 3;
    let map = RangeMap::new(Range::new(5, size), Range::new(10, size));

    assert_eq!(4, map.map(4));
    assert_eq!(8, map.map(8));
    assert_eq!(10, map.map(10));
}
