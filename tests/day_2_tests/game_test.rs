use std::collections::HashMap;

use advent_of_code_2023::day_2::{cube::Cube, game::Game, viewing::Viewing};

#[test]
pub fn new_cube_count_makes_new() {
    let mut totals = HashMap::<Cube, u32>::new();

    totals.insert(Cube::Green, 5);
    totals.insert(Cube::Blue, 10);

    let mut viewing_1 = HashMap::<Cube, u32>::new();

    viewing_1.insert(Cube::Blue, 4);
    viewing_1.insert(Cube::Red, 1);

    let mut viewing_2 = HashMap::<Cube, u32>::new();

    viewing_2.insert(Cube::Blue, 1);

    let viewings = vec![Viewing::new(&viewing_1), Viewing::new(&viewing_2)];

    let actual = Game::new(&totals, &viewings);

    assert_eq!(totals, actual.totals);
}
