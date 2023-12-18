use super::{direction::Direction, position::Position};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Beam {
    pub direction: Direction,
    pub position: Position,
}

impl Beam {
    pub fn new(direction: Direction, position: Position) -> Self {
        return Beam {
            direction,
            position,
        };
    }

    pub fn translate(&self) -> Self {
        return Beam::new(
            self.direction,
            self.position.translate(self.direction),
        );
    }
}
