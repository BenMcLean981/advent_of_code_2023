use crate::{day_3::grid::Grid, utils::file_utils::read_lines};

pub fn solve() {
    let filename = "src/day_3/sample.txt";

    let lines = read_lines(filename);
    let grid = Grid::from_lines(lines);

    println!("Day 3");
}
