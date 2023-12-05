use crate::{
    day_5::{seed_data::SeedData, seed_maps::SeedMaps},
    utils::file_utils::read_lines,
};

use std::str::FromStr;

use super::{range::Range, seed_data_range::SeedDataRange};

pub fn solve() {
    let filename = "src/day_5/input.txt";

    let binding = read_lines(filename);
    let lines: Vec<&str> = binding.iter().map(|s| s.as_str()).collect();

    let part_1 = solve_part_1(&lines);
    let part_2 = solve_part_2(&lines);

    println!("Day 5");
    println!("The closest location number for part 1 is {part_1}.");
    println!("The closest location number for part 1 is {part_2}.");
}

fn solve_part_1(lines: &Vec<&str>) -> u64 {
    let seeds: Vec<u64> = lines[0]
        .split(' ')
        .filter(|s| !s.trim().is_empty())
        .skip(1)
        .map(|s| u64::from_str(s).unwrap())
        .collect();

    let seed_maps = get_seed_maps(&lines);

    let seed_datas = seeds
        .iter()
        .map(|s| seed_maps.get_data(*s))
        .collect::<Vec<SeedData>>();

    return seed_datas.iter().map(|d| d.location).min().unwrap();
}

fn solve_part_2(lines: &Vec<&str>) -> u64 {
    let seeds = get_seed_ranges(lines[0]);
    let seed_maps = get_seed_maps(&lines);

    let seed_datas = seeds
        .iter()
        .map(|s| seed_maps.get_data_range(*s))
        .collect::<Vec<SeedDataRange>>();

    return seed_datas
        .iter()
        .flat_map(|d| d.location.iter().map(|r| r.lower))
        .min()
        .unwrap();
}

fn get_seed_ranges(line: &str) -> Vec<Range> {
    let numbers = line
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| u64::from_str(s).unwrap())
        .collect::<Vec<u64>>();

    let mut results: Vec<Range> = vec![];

    for i in (0..numbers.len()).step_by(2) {
        let lower = numbers[i];
        let size = numbers[i + 1];

        results.push(Range::new(lower, size));
    }

    return results;
}

fn get_seed_maps(lines: &Vec<&str>) -> SeedMaps {
    let seed_map_lines = lines[2..].to_vec();
    let seed_maps = SeedMaps::from_lines(seed_map_lines);
    seed_maps
}
