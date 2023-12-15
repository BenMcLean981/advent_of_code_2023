use crate::{day_13::reflection::Reflection, utils::file_utils::read_lines};

use super::reflection::Mirror;

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
    let part_2 = solve_part_2(&reflections);

    println!("Day 13");
    println!("The summary of my notes gives me {part_1}.");
    println!("After fixing smudges I get me {part_2}.");
}

fn solve_part_1(reflections: &Vec<Reflection>) -> usize {
    return reflections.iter().map(|r| get_part_1_answer(r)).sum();
}

fn get_part_1_answer(reflection: &Reflection) -> usize {
    let mirror = reflection.find_mirror();

    return convert_mirror(&mirror.unwrap());
}

fn solve_part_2(reflections: &Vec<Reflection>) -> usize {
    return reflections.iter().map(|r| get_part_2_answer(r)).sum();
}

fn get_part_2_answer(reflection: &Reflection) -> usize {
    let replacements = reflection.make_alternatives();
    let wrong_answer = reflection.find_mirror().unwrap();

    for r in replacements {
        let result = override_wrong_answer(&r, &wrong_answer);

        if result.is_some() {
            return result.unwrap();
        }
    }

    panic!();
}

fn override_wrong_answer(
    reflection: &Reflection,
    wrong_answer: &Mirror,
) -> Option<usize> {
    let mirror = reflection.find_mirror_with_override(wrong_answer);

    if mirror.is_none() || mirror.unwrap() == *wrong_answer {
        return None;
    }

    return Some(convert_mirror(&mirror.unwrap()));
}

fn convert_mirror(mirror: &Mirror) -> usize {
    match mirror {
        super::reflection::Mirror::Vertical(col) => *col,
        super::reflection::Mirror::Horizontal(row) => row * 100,
    }
}
