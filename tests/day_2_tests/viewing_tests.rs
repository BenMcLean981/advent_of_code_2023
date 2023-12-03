use std::{collections::HashMap, str::FromStr};

use advent_of_code_2023::day_2::{cube::Cube, viewing::Viewing};

#[test]
pub fn new_cube_count_makes_new() {
    let mut counts = HashMap::<Cube, u32>::new();

    counts.insert(Cube::Green, 5);
    counts.insert(Cube::Blue, 10);

    let actual = Viewing::new(&counts);

    assert_eq!(counts, actual.counts);
}

#[test]
pub fn from_str_one_cube_makes_viewing() {
    let actual = Viewing::from_str("5 blue").unwrap();
    let mut expected_map = HashMap::<Cube, u32>::new();

    expected_map.insert(Cube::Blue, 5);

    let expected = Viewing::new(&expected_map);

    assert_eq!(expected, actual);
}

#[test]
pub fn from_str_many_cubes_makes_viewing() {
    let actual = Viewing::from_str("5 blue, 1 red, 3 green").unwrap();
    let mut expected_map = HashMap::<Cube, u32>::new();

    expected_map.insert(Cube::Blue, 5);
    expected_map.insert(Cube::Red, 1);
    expected_map.insert(Cube::Green, 3);

    let expected = Viewing::new(&expected_map);

    assert_eq!(expected, actual);
}
