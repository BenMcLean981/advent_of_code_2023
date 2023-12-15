use crate::{day_13::reflection::Reflection, utils::file_utils::read_lines};

pub fn solve() {
    let filename = "src/day_13/input.txt";

    let binding = read_lines(filename);
    let lines = binding.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    let reflections = lines
        .split(|l| l.is_empty())
        .map(|g| g.to_vec())
        .map(|lines| Reflection::from_lines(lines))
        .collect::<Vec<Reflection>>();

    let part_1 = solve_part_1(&reflections);

    println!("Day 13");
    println!("The summary of my notes gives me {part_1}.")
}

fn solve_part_1(reflections: &Vec<Reflection>) -> usize {
    return reflections.iter().map(|r| get_part_1_answer(r)).sum();
}

fn get_part_1_answer(reflection: &Reflection) -> usize {
    let mirror = reflection.find_mirror();

    match mirror {
        super::reflection::Mirror::Vertical(col) => col,
        super::reflection::Mirror::Horizontal(row) => row * 100,
    }
}
