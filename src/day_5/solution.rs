use crate::{
    day_5::{seed_data::SeedData, seed_maps::SeedMaps},
    utils::file_utils::read_lines,
};

use std::str::FromStr;

pub fn solve() {
    let filename = "src/day_5/input.txt";

    let binding = read_lines(filename);
    let lines: Vec<&str> = binding.iter().map(|s| s.as_str()).collect();

    let seeds: Vec<u32> = lines[0]
        .split(' ')
        .filter(|s| !s.trim().is_empty())
        .skip(1)
        .map(|s| u32::from_str(s).unwrap())
        .collect();

    let seed_map_lines = lines[2..].to_vec();
    let seed_maps = SeedMaps::from_lines(seed_map_lines);

    let seed_datas = seeds
        .iter()
        .map(|s| seed_maps.get_data(*s))
        .collect::<Vec<SeedData>>();

    let closest = seed_datas.iter().map(|d| d.location).min().unwrap();

    println!("Day 5");
    println!("The closest location number is {closest}.");
}
