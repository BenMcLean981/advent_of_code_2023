#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Position {
    pub row: i32,
    pub col: i32,
}

impl Position {
    pub fn new(row: i32, col: i32) -> Self {
        return Position { row, col };
    }

    pub fn get_manhattan_distance(&self, other: &Position) -> u32 {
        return self.row.abs_diff(other.row) + self.col.abs_diff(other.col);
    }
}
