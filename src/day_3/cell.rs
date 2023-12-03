use super::position::Position;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Cell {
    pub position: Position,
    pub char: char,
}

impl Cell {
    pub fn new(position: Position, char: char) -> Self {
        return Cell { position, char };
    }

    pub fn is_symbol(&self) -> bool {
        return !self.is_number() && self.char != '.';
    }

    pub fn is_number(&self) -> bool {
        return self.char.is_digit(10);
    }
}
