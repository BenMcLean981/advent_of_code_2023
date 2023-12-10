use std::str::FromStr;

use crate::{
    day_8::{direction::Direction, edge::Edge, map::Map},
    utils::file_utils::read_lines,
};

pub fn solve() {
    let filename = "src/day_8/input.txt";

    let lines = read_lines(filename);
    let directions = lines[0]
        .chars()
        .map(Direction::from)
        .collect::<Vec<Direction>>();
    let edges = lines
        .iter()
        .skip(2)
        .map(|l| Edge::from_str(l).unwrap())
        .collect::<Vec<Edge>>();
    let map = Map::from_edges(&edges);

    let num_directions_followed = count_directions_followed(map, directions);

    println!("Day 8");
    println!("The number of directions to leave the desert is {num_directions_followed}.");
}

fn count_directions_followed(map: Map, directions: Vec<Direction>) -> u32 {
    let mut node = "AAA";
    let mut count = 0;

    while node != "ZZZ" && count < 100_000 {
        let direction_index = count % directions.len();
        let direction = directions[direction_index];

        node = map.get_next(node, direction);
        count += 1;
    }

    return count as u32;
}
