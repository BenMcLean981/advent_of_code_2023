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
    let poly_1 = make_polygon_1(&commands);
    let poly_2 = make_polygon_2(&commands);

    let area_1 = poly_1.get_area();
    let area_2 = poly_2.get_area();

    println!("Day 18");
    println!("The area of the trench is {area_1}.");
    println!("The area of the second trench is {area_2}.");
}

fn make_polygon_1(commands: &Vec<Command>) -> Polygon {
    let mut points = vec![Xy::new(0, 0)];

    for command in commands {
        let last = points.last().unwrap();
        let vec = command.to_vector_1();

        let point = last.add(&vec);

        points.push(point);
    }

    return Polygon::new(points);
}

fn make_polygon_2(commands: &Vec<Command>) -> Polygon {
    let mut points = vec![Xy::new(0, 0)];

    for command in commands {
        let last = points.last().unwrap();
        let vec = command.to_vector_2();

        let point = last.add(&vec);

        points.push(point);
    }

    return Polygon::new(points);
}
