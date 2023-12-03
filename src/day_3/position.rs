#[derive(PartialEq, Eq, Hash)]
pub struct Position(u32, u32);

impl Position {
    pub fn new(row: u32, col: u32) -> Self {
        return Position(row, col);
    }
}
