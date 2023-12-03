#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position(u32, u32);

impl Position {
    pub fn new(row: u32, col: u32) -> Self {
        return Position(row, col);
    }

    pub fn is_adjacent(&self, p: Position) -> bool {
        let diff = self.1.abs_diff(p.1);

        return diff == 1;
    }

    pub fn is_neighboring(&self, p: Position) -> bool {
        let row_diff = self.0.abs_diff(p.0);
        let col_diff = self.1.abs_diff(p.1);

        return row_diff <= 1 && col_diff <= 1;
    }
}
