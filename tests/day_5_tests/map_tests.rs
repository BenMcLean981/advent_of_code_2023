use advent_of_code_2023::day_5::{map::Map, range::Range, range_map::RangeMap};

#[test]
pub fn map_in_no_range_does_not_map() {
    let map = Map::new(vec![
        RangeMap::new(Range::new(5, 3), Range::new(50, 3)),
        RangeMap::new(Range::new(45, 5), Range::new(0, 5)),
    ]);

    assert_eq!(8, map.map(8));
    assert_eq!(44, map.map(44));
    assert_eq!(50, map.map(50));
}

#[test]
pub fn map_in_range_maps() {
    let map = Map::new(vec![
        RangeMap::new(Range::new(5, 3), Range::new(50, 3)),
        RangeMap::new(Range::new(47, 5), Range::new(5, 5)),
    ]);

    assert_eq!(50, map.map(5));
    assert_eq!(51, map.map(6));
    assert_eq!(52, map.map(7));

    assert_eq!(5, map.map(47));
    assert_eq!(6, map.map(48));
    assert_eq!(7, map.map(49));
    assert_eq!(8, map.map(50));
    assert_eq!(9, map.map(51));
}

#[test]
pub fn map_from_lines_makes_map() {
    let lines = vec![
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
    ];

    let actual = Map::from_lines(&lines);
    let expected = Map::new(vec![
        RangeMap::new(Range::new(77, 23), Range::new(45, 23)),
        RangeMap::new(Range::new(45, 19), Range::new(81, 19)),
        RangeMap::new(Range::new(64, 13), Range::new(68, 13)),
    ]);

    assert_eq!(expected, actual);
}

#[test]
pub fn map_ranges_no_intersection_returns_empty() {
    let map = Map::new(vec![
        RangeMap::new(Range::new(77, 23), Range::new(45, 23)),
        RangeMap::new(Range::new(45, 19), Range::new(81, 19)),
        RangeMap::new(Range::new(64, 13), Range::new(68, 13)),
    ]);

    assert_eq!(true, map.map_range(Range::new(5, 2)).is_empty());
}

#[test]
pub fn map_range_subset_of_one_returns_one() {
    let map = Map::new(vec![
        RangeMap::new(Range::new(77, 23), Range::new(45, 23)),
        RangeMap::new(Range::new(45, 19), Range::new(81, 19)),
        RangeMap::new(Range::new(64, 13), Range::new(68, 13)),
    ]);

    let range = Range::new(65, 10);

    assert_eq!(vec![range], map.map_range(range));
}

#[test]
pub fn map_range_partial_subset_of_one_returns_two() {
    let map = Map::new(vec![
        RangeMap::new(Range::new(77, 23), Range::new(45, 23)),
        RangeMap::new(Range::new(45, 19), Range::new(81, 19)),
        RangeMap::new(Range::new(64, 13), Range::new(68, 13)),
    ]);

    let actual = map.map_range(Range::new(60, 10));
    let expected = vec![Range::new(60, 4), Range::new(68, 6)];

    assert_eq!(true, have_same_items(actual, expected));
}

#[test]
pub fn map_range_partial_subset_of_two_returns_three() {
    let map = Map::new(vec![
        RangeMap::new(Range::new(77, 23), Range::new(45, 23)),
        RangeMap::new(Range::new(45, 19), Range::new(81, 19)),
        RangeMap::new(Range::new(66, 13), Range::new(68, 13)),
    ]);

    let actual = map.map_range(Range::new(55, 15));
    let expected =
        vec![Range::new(81, 9), Range::new(64, 2), Range::new(68, 4)];

    assert_eq!(true, have_same_items(actual, expected));
}

fn have_same_items(s1: Vec<Range>, s2: Vec<Range>) -> bool {
    let sorted_1 = sort(s1);
    let sorted_2 = sort(s2);

    return sorted_1.eq(&sorted_2);
}

fn sort(vec: Vec<Range>) -> Vec<Range> {
    let mut result = vec.to_vec();

    result.sort();

    return result;
}
