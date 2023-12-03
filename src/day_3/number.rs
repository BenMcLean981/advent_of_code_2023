use std::collections::HashSet;

use super::position::Position;

pub struct Number {
    pub number: u32,
    positions: HashSet<Position>,
}

impl Number {
    pub fn new(number: u32, positions: HashSet<Position>) -> Self {
        return Number { number, positions };
    }

    pub fn is_neighboring(&self, p: Position) -> bool {
        return self.positions.iter().any(|pos| pos.is_neighboring(p));
    }
}
