use std::str::FromStr;

use crate::{
    day_8::{direction::Direction, edge::Edge, map::Map},
    utils::file_utils::read_lines,
};

pub fn solve() {
    let filename = "src/day_8/input.txt";

    let lines = read_lines(filename);
    let directions = lines[0]
        .chars()
        .map(Direction::from)
        .collect::<Vec<Direction>>();
    let edges = lines
        .iter()
        .skip(2)
        .map(|l| Edge::from_str(l).unwrap())
        .collect::<Vec<Edge>>();
    let map = Map::from_edges(&edges);

    let num_directions_followed =
        count_directions_followed(&map, &directions, "AAA", "ZZZ");
    let all_followed = count_multiple_directions_followed(&map, &directions);

    println!("Day 8");
    println!("The number of directions to leave the desert is {num_directions_followed}.");
    println!("The number of directions to leave the desert as a ghost is {all_followed}.");
}

fn count_directions_followed(
    map: &Map,
    directions: &Vec<Direction>,
    start: &str,
    end: &str,
) -> u64 {
    let mut node = start;
    let mut count = 0;

    // it is possible that this doesn't work for part 1
    // for example a node with BZZZ would count as the end.
    // in my case at least the inputs don't cause this.

    while !node.ends_with(end) {
        let direction_index = count % directions.len();
        let direction = directions[direction_index];

        node = map.get_next(node, direction);
        count += 1;

        if count >= 100_000 {
            panic!();
        }
    }

    return count as u64;
}

fn count_multiple_directions_followed(
    map: &Map,
    directions: &Vec<Direction>,
) -> u64 {
    let nodes = map.get_starts();
    let counts = nodes
        .iter()
        .map(|n| count_directions_followed(map, directions, n, "Z"))
        .collect();

    return multiple_lcm(counts);
}

fn multiple_lcm(nums: Vec<u64>) -> u64 {
    let mut result = nums[0];

    for num in nums.iter().skip(1) {
        result = lcm(result, *num);
    }

    return result;
}

fn lcm(a: u64, b: u64) -> u64 {
    return a * b / gcd(a, b);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}
