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
