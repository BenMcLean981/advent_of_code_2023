use crate::{day_10::grid::Grid, utils::file_utils::read_lines};

use super::{area_calculator::AreaCalculator, loop_finder::LoopFinder};

pub fn solve() {
    let filename = "src/day_10/input.txt";

    let lines = read_lines(filename);
    let grid = Grid::from_lines(lines.iter().map(|s| s.as_str()).collect());

    let half_loop_length = count_loop_length(&grid) / 2;
    let area = count_area(&grid);

    println!("Day 10");
    println!("The furthest distance in the loop is {half_loop_length}.");
    println!("The area inside the loop is {area}.");
}

fn count_loop_length(grid: &Grid) -> u32 {
    return LoopFinder::find_loop(grid).len() as u32;
}

fn count_area(grid: &Grid) -> u32 {
    let path = LoopFinder::find_loop(grid);

    return AreaCalculator::new(grid, path).get_area();
}
