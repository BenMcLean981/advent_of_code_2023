use std::collections::BinaryHeap;

use crate::day_17::heat_map::HeatMap;

use super::{path::Path, position::Position};

pub struct DijsktraSolver {
    map: HeatMap,
    queue: BinaryHeap<State>,
}

impl DijsktraSolver {
    pub fn new(map: HeatMap) -> Self {
        let mut queue = BinaryHeap::new();

        queue.push(State::make_initial());

        return DijsktraSolver { map, queue };
    }

    pub fn pop_min(&mut self) -> State {
        return self.queue.pop().unwrap();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    pub position: Position,
    pub loss: usize,
    pub path: Path,
}

impl State {
    pub fn make_initial() -> Self {
        return State {
            position: Position::new(0, 0),
            loss: 0,
            path: Path::from_positions(vec![Position::new(0, 0)]),
        };
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.loss.cmp(&other.loss));
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(other).unwrap();
    }
}
