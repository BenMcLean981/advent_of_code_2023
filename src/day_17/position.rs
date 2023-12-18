use super::direction::Direction;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Position {
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
