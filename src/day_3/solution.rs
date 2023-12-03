use crate::{day_3::grid::Grid, utils::file_utils::read_lines};

pub fn solve() {
    let filename = "src/day_3/input.txt";

    let lines = read_lines(filename);
    let grid = Grid::from_lines(lines);

    let part_numbers = grid.get_part_numbers();
    let gear_ratios = grid.get_gear_ratios();

    let sum = part_numbers.iter().fold(0, |sum, n| sum + n);
    let gear_ratio_sum = gear_ratios.iter().fold(0, |sum, gr| sum + gr);

    println!("Day 3");
    println!("The sum of part nubmers is {sum}.");
    println!("The sum of gear ratios is {gear_ratio_sum}.");
}
