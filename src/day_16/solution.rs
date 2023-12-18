use crate::{
    day_16::{
        beam::Beam, direction::Direction, mirror_grid::MirrorGrid,
        position::Position,
    },
    utils::file_utils::read_lines,
};

pub fn solve() {
    let filename = "src/day_16/input.txt";

    let binding = read_lines(filename);
    let lines = binding.iter().map(|l| l.as_str()).collect();
    let mut grid = MirrorGrid::from_lines(lines);
    grid.simulate(Beam::new(Direction::Right, Position::new(0, 0)));

    let energized = grid.count_energized();
    let best = grid.count_best_energized();

    println!("Day 16");
    println!("The number of energized cells is: {energized}.");
    println!("The maximum number of energized cells is: {best}.");
}
