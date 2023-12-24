use std::str::FromStr;

use crate::{day_18::command::Command, utils::file_utils::read_lines};

use super::{polygon::Polygon, xy::Xy};

pub fn solve() {
    let filename = "src/day_18/input.txt";

    let binding = read_lines(filename);
    let lines: Vec<&str> = binding.iter().map(|s| s.as_str()).collect();
    let commands: Vec<Command> = lines
        .iter()
        .map(|s| Command::from_str(*s).unwrap())
        .collect();
    let polygon = make_polygon(commands);
    let area = polygon.get_area();

    println!("Day 18");
    println!("The area of the trench is {area}.");
}

fn make_polygon(commands: Vec<Command>) -> Polygon {
    let mut points = vec![Xy::new(0, 0)];

    for command in commands {
        let last = points.last().unwrap();
        let vec = command.to_vector();

        let point = last.add(&vec);

        points.push(point);
    }

    return Polygon::new(points);
}
