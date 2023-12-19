use super::position::Position;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    Right,
    Down,
    Up,
    Left,
}

impl Direction {
    pub fn make_delta(from: Position, to: Position) -> Self {
        let adjacent_rows = from.row.abs_diff(to.row) == 1;
        let adjacent_cols = from.col.abs_diff(to.col) == 1;

        let same_rows = from.row.abs_diff(to.row) == 0;
        let same_cols = from.col.abs_diff(to.col) == 0;

        if !(adjacent_rows && same_cols) && !(adjacent_cols && same_rows) {
            panic!();
        }

        if to.row > from.row {
            return Direction::Down;
        } else if to.row < from.row {
            return Direction::Up;
        } else if to.col > from.col {
            return Direction::Right;
        } else {
            return Direction::Left;
        }
    }
}
