use std::str::FromStr;

use crate::{day_2::game::Game, utils::file_utils};

pub fn solve() {
    let filename = "src/day_2/input.txt";

    let lines = file_utils::read_lines(filename);
    let games: Vec<Game> = lines
        .iter()
        .map(|l| Game::from_str(&l))
        .map(|g| g.unwrap())
        .collect();

    let valid_games: Vec<&Game> =
        games.iter().filter(|g| g.is_possible()).collect();

    let sum = valid_games.iter().fold(0, |sum, g| sum + g.id);
    let power_sum = games.iter().fold(0, |sum, g| sum + g.get_minimum_power());

    println!("Day 2");
    println!("The sum of valid ids is {}.", sum);
    println!("The power sum is {}.", power_sum);
}
