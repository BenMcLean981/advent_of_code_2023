use super::direction::Direction;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position {
    pub row: i32,
    pub col: i32,
}

impl Position {
    pub fn new(row: i32, col: i32) -> Self {
        return Position { row, col };
    }

    pub fn translate(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Position::new(self.row - 1, self.col),
            Direction::Down => Position::new(self.row + 1, self.col),
            Direction::Left => Position::new(self.row, self.col - 1),
            Direction::Right => Position::new(self.row, self.col + 1),
        }
    }
}
