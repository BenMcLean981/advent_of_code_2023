use super::direction::Direction;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position {
    pub row: i32,
    pub col: i32,
}

impl Position {
    pub fn new(row: i32, col: i32) -> Position {
        return Position { row, col };
    }

    pub fn move_by(&self, d: Direction) -> Position {
        match d {
            Direction::Right => Position::new(self.row, self.col + 1),
            Direction::Down => Position::new(self.row + 1, self.col),
            Direction::Up => Position::new(self.row - 1, self.col),
            Direction::Left => Position::new(self.row, self.col - 1),
        }
    }
}
