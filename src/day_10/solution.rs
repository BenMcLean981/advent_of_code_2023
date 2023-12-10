use crate::{day_10::grid::Grid, utils::file_utils::read_lines};

use super::loop_finder::LoopFinder;

pub fn solve() {
    let filename = "src/day_10/input.txt";

    let lines = read_lines(filename);
    let grid = Grid::from_lines(lines.iter().map(|s| s.as_str()).collect());
    let half_loop_length = count_loop_length(&grid) / 2;

    println!("Day 10");
    println!("The furthest distance in the loop is {half_loop_length}.")
}

fn count_loop_length(grid: &Grid) -> u32 {
    return LoopFinder::find_loop(grid).len() as u32;
}
