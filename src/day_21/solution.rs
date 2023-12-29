use crate::utils::file_utils::read_lines;

use super::graph::Graph;

const SMALL_STEPS: u32 = 64;
const INF_STEPS: u32 = 26_501_365;

pub fn solve() {
    let filename = "src/day_21/input.txt";

    let lines = read_lines(filename);
    let lines = lines.iter().map(|s| s.as_str()).collect();

    let graph = Graph::from_lines(&lines);

    let tiles = graph.step(SMALL_STEPS).len();
    let inf_tiles = graph.step_infinitely(INF_STEPS).len();

    println!(
        "The number of garden tiles reachable in {SMALL_STEPS} steps is {tiles}."
    );

    println!(
        "The number of garden tiles reachable on an infinite plane in {INF_STEPS} steps is {inf_tiles}."
    );
}
