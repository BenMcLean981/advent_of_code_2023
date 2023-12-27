use crate::utils::file_utils::read_lines;

use super::{graph::Graph, signal::Signal, simulator::Simulator};

pub fn solve() {
    let filename = "src/day_20/input.txt";

    let graph = build_graph(filename);
    let count = simulate(graph);

    println!("Day 20");
    println!("The pulse count total is {count}.");
}

const NUM_PRESSES: usize = 1_000;

fn simulate(graph: Graph) -> u32 {
    let mut simulator = Simulator::new(graph);

    for _ in 0..NUM_PRESSES {
        simulator.simulate();
    }

    let lows = simulator
        .processed
        .iter()
        .filter(|m| m.signal == Signal::Low)
        .count() as u32;

    let highs = simulator
        .processed
        .iter()
        .filter(|m| m.signal == Signal::High)
        .count() as u32;

    return lows * highs;
}

fn build_graph(filename: &str) -> Graph {
    let lines = read_lines(filename);
    let mut graph = Graph::new();

    for l in lines {
        graph.add(l.as_str());
    }

    graph.connect();

    return graph;
}
