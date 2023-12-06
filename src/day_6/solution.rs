use std::str::FromStr;

use crate::utils::file_utils::read_lines;

use super::race::Race;

pub fn solve() {
    let filename = "src/day_6/input.txt";

    let lines = read_lines(filename);
    let lines = lines.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    let part_1 = solve_part_1(&lines);
    let part_2 = solve_part_2(&lines);

    println!("Day 6");
    println!(
        "The product of the number of possible wins for each race is {part_1}."
    );

    println!(
        "The product of the number of possible wins for the one singular race is {part_2}."
    );
}

fn solve_part_1(lines: &Vec<&str>) -> u64 {
    let races = get_races(lines);

    let num_possible = races
        .iter()
        .map(|r| r.get_num_possible_wins())
        .collect::<Vec<u64>>();

    return num_possible.iter().product();
}

fn get_races(lines: &Vec<&str>) -> Vec<Race> {
    let times = read_line(lines[0]);
    let distances = read_line(lines[1]);

    if times.len() != distances.len() {
        panic!();
    }

    let mut result: Vec<Race> = vec![];

    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];

        result.push(Race::new(time, distance))
    }

    return result;
}

fn read_line(line: &str) -> Vec<u64> {
    return line
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| u64::from_str(s).unwrap())
        .collect();
}

fn solve_part_2(lines: &Vec<&str>) -> u64 {
    let race = get_race(lines);

    return race.get_num_possible_wins();
}

fn get_race(lines: &Vec<&str>) -> Race {
    let time = read_single_number_line(lines[0]);
    let distance = read_single_number_line(lines[1]);

    return Race::new(time, distance);
}

fn read_single_number_line(line: &str) -> u64 {
    let digits = line.chars().filter(|c| c.is_digit(10));
    let digits = digits.collect::<String>();

    return u64::from_str(digits.as_str()).unwrap();
}
