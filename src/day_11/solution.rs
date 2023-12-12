use crate::{day_11::universe::Universe, utils::file_utils::read_lines};

pub fn solve() {
    let filename = "src/day_11/input.txt";

    let binding = read_lines(filename);
    let lines = binding.iter().map(|s| s.as_str()).collect();
    let universe = Universe::from_lines(lines).expand();

    let distance = universe.get_sum_distances(1);

    println!("Day 11");
    println!("The sum of the distances between galaxies is {distance}.");
}
