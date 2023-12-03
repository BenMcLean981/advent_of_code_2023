use std::collections::HashSet;

use super::position::Position;

pub struct Number {
    number: u32,
    positions: HashSet<Position>,
}

impl Number {
    pub fn new(number: u32, positions: HashSet<Position>) -> Self {
        return Number { number, positions };
    }
}
