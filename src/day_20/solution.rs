use std::collections::VecDeque;

use crate::utils::file_utils::read_lines;

use super::{graph::Graph, signal::Signal, simulator::Simulator};

pub fn solve() {
    let filename = "src/day_20/input.txt";

    let graph = build_graph(filename);
    let count = simulate(&graph);
    let count_rx = simulate_rx(&graph);

    println!("Day 20");
    println!("The pulse count total is {count}.");
    println!("The number of button presses until rx gets low is {count_rx}.");
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

const NUM_PRESSES: usize = 1_000;

fn simulate(graph: &Graph) -> u32 {
    let mut processed = VecDeque::new();
    let mut simulator = Simulator::new(graph.clone());

    for _ in 0..NUM_PRESSES {
        processed.extend(simulator.simulate_button());
    }

    let lows =
        processed.iter().filter(|m| m.signal == Signal::Low).count() as u32;

    let highs = processed
        .iter()
        .filter(|m| m.signal == Signal::High)
        .count() as u32;

    return lows * highs;
}

fn simulate_rx(graph: &Graph) -> u64 {
    let mut simulator = Simulator::new(graph.clone());

    return simulator.simulate_until_rx();
}
