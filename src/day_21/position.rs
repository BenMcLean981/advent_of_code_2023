use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position {
    pub col: i32,
    pub row: i32,
}

impl Position {
    pub fn new(row: i32, col: i32) -> Position {
        return Position { col, row };
    }

    pub fn get_neighbors(&self) -> HashSet<Position> {
        return vec![
            Position::new(self.row - 1, self.col),
            Position::new(self.row + 1, self.col),
            Position::new(self.row, self.col - 1),
            Position::new(self.row, self.col + 1),
        ]
        .iter()
        .copied()
        .collect();
    }
}
