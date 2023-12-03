use crate::{day_3::grid::Grid, utils::file_utils::read_lines};

pub fn solve() {
    let filename = "src/day_3/input.txt";

    let lines = read_lines(filename);
    let grid = Grid::from_lines(lines);
    let part_numbers = grid.get_part_numbers();

    let sum = part_numbers.iter().fold(0, |sum, n| sum + n);

    println!("Day 3");
    println!("The sum of part nubmers is {sum}.")
}
