use std::borrow::Borrow;

use crate::{
    day_17::{
        dijkstra_solver::{DijsktraSolver, State},
        heat_map::HeatMap,
        path::Path,
    },
    utils::file_utils::read_lines,
};

pub fn solve() {
    let filename = "src/day_17/sample.txt";

    let binding = read_lines(filename);
    let lines = binding.iter().map(|s| s.as_str()).collect();

    let heat_map = HeatMap::from_lines(lines);
    let destination = heat_map.get_bot_right();

    let mut solver = DijsktraSolver::new(heat_map.clone());

    let state: State = solver.solve(destination);
    let path = extract_path(&state);

    heat_map.debug_path(path);

    let loss = state.loss;

    println!("Day 17");
    println!("The loss of the shortest path is: {loss}.");
}

fn extract_path(state: &State) -> Path {
    let mut positions = vec![];

    let mut next = Box::new(Some(state.clone()));

    while let Some(n) = next.borrow() {
        positions.push(n.position);
        next = n.previous.clone();
    }

    return Path::from_positions(positions.iter().rev().copied().collect());
}
