use advent_of_code_2023::day_8::{direction::Direction, edge::Edge, map::Map};

#[test]
pub fn from_edges_makes_map() {
    let actual = Map::from_edges(vec![
        &Edge::new("AAA", "BBB", "CCC"),
        &Edge::new("BBB", "DDD", "EEE"),
        &Edge::new("CCC", "ZZZ", "GGG"),
        &Edge::new("DDD", "DDD", "DDD"),
        &Edge::new("EEE", "EEE", "EEE"),
        &Edge::new("GGG", "GGG", "GGG"),
        &Edge::new("ZZZ", "ZZZ", "ZZZ"),
    ]);

    assert_eq!("BBB", actual.get_next("AAA", Direction::Left));
    assert_eq!("CCC", actual.get_next("AAA", Direction::Right));

    assert_eq!("DDD", actual.get_next("BBB", Direction::Left));
    assert_eq!("EEE", actual.get_next("BBB", Direction::Right));

    assert_eq!("ZZZ", actual.get_next("CCC", Direction::Left));
    assert_eq!("GGG", actual.get_next("CCC", Direction::Right));

    assert_eq!("DDD", actual.get_next("DDD", Direction::Left));
    assert_eq!("DDD", actual.get_next("DDD", Direction::Right));
}
