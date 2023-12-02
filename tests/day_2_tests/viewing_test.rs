use std::collections::HashMap;

use advent_of_code_2023::day_2::{cube::Cube, viewing::Viewing};

#[test]
pub fn new_cube_count_makes_new() {
    let mut counts = HashMap::<Cube, u32>::new();

    counts.insert(Cube::Green, 5);
    counts.insert(Cube::Blue, 10);

    let actual = Viewing::new(&counts);

    assert_eq!(counts, actual.counts);
}
