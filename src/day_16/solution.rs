use crate::{day_16::mirror_grid::MirrorGrid, utils::file_utils::read_lines};

pub fn solve() {
    let filename = "src/day_16/input.txt";

    let binding = read_lines(filename);
    let lines = binding.iter().map(|l| l.as_str()).collect();
    let mut grid = MirrorGrid::from_lines(lines);
    grid.simulate();

    let energized = grid.count_energized();

    println!("Day 16");
    println!("The number of energized cells is: {energized}.")
}
