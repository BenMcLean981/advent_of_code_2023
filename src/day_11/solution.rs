use crate::{day_11::universe::Universe, utils::file_utils::read_lines};

pub fn solve() {
    let filename = "src/day_11/input.txt";

    let binding = read_lines(filename);
    let lines = binding.iter().map(|s| s.as_str()).collect();
    let universe = Universe::from_lines(lines);

    let d = universe.get_sum_distances(1);
    let big_d = universe.get_sum_distances(1_000_000);

    println!("Day 11");
    println!("The sum of the distances between galaxies with is {d}.");
    println!("The sum of the distances between galaxies with 1M expansions is {big_d}.");
}
