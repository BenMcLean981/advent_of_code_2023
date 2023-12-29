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
    let inf_tiles = solve_part_2(&graph);

    println!(
        "The number of garden tiles reachable in {SMALL_STEPS} steps is {tiles}."
    );

    println!(
        "The number of garden tiles reachable on an infinite plane in {INF_STEPS} steps is {inf_tiles}."
    );
}

fn solve_part_2(graph: &Graph) -> f64 {
    let stepped = graph.step_infinitely(327);

    let (x0, y0) = (65 as f64, stepped[65] as f64);
    let (x1, y1) = (196 as f64, stepped[196] as f64);
    let (x2, y2) = (327 as f64, stepped[327] as f64);

    let compute = |n: f64| -> f64 {
        let y01 = (y1 - y0) / (x1 - x0);
        let y12 = (y2 - y1) / (x2 - x1);
        let y012 = (y12 - y01) / (x2 - x0);

        return y012 * (n - x0) * (n - x1) + y01 * (n - x0) + y0;
    };

    // sanity check
    assert_eq!(y0, compute(x0));
    assert_eq!(y1, compute(x1));
    assert_eq!(y2, compute(x2));

    let n = 26_501_365.0;

    return compute(n);
}
