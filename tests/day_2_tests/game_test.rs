use std::{collections::HashMap, str::FromStr};

use advent_of_code_2023::day_2::{
    cube::Cube,
    game::{get_default_totals, Game},
    viewing::Viewing,
};

#[test]
pub fn new_cube_count_makes_new() {
    let id = 31;
    let mut totals = HashMap::<Cube, u32>::new();

    totals.insert(Cube::Green, 5);
    totals.insert(Cube::Blue, 10);

    let mut viewing_1 = HashMap::<Cube, u32>::new();

    viewing_1.insert(Cube::Blue, 4);
    viewing_1.insert(Cube::Red, 1);

    let mut viewing_2 = HashMap::<Cube, u32>::new();

    viewing_2.insert(Cube::Blue, 1);

    let viewings = vec![Viewing::new(&viewing_1), Viewing::new(&viewing_2)];

    let actual = Game::new(id, &totals, &viewings);

    assert_eq!(id, actual.id);
    assert_eq!(totals, actual.totals);
    assert_eq!(viewings, actual.viewings);
}

#[test]
pub fn from_str_no_viewings_returns_totals_only() {
    let actual = Game::from_str("Game 5:").unwrap();
    let expected = Game::new(5, &get_default_totals(), &vec![]);

    assert_eq!(expected, actual);
}

#[test]
pub fn from_str_one_viewing_returns_game() {
    let actual = Game::from_str("Game 5: 1 blue").unwrap();

    let mut viewing_1 = HashMap::<Cube, u32>::new();
    viewing_1.insert(Cube::Blue, 1);

    let expected =
        Game::new(5, &get_default_totals(), &vec![Viewing::new(&viewing_1)]);

    assert_eq!(expected, actual);
}

#[test]
pub fn from_str_one_severals_returns_game() {
    let actual = Game::from_str("Game 5: 1 blue; 3 red, 5 green").unwrap();

    let mut viewing_1 = HashMap::<Cube, u32>::new();
    viewing_1.insert(Cube::Blue, 1);

    let mut viewing_2 = HashMap::<Cube, u32>::new();
    viewing_2.insert(Cube::Red, 3);
    viewing_2.insert(Cube::Green, 5);

    let expected = Game::new(
        5,
        &get_default_totals(),
        &vec![Viewing::new(&viewing_1), Viewing::new(&viewing_2)],
    );

    assert_eq!(expected, actual);
}
