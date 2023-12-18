#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_downwards(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        }
    }

    pub fn rotate_upwards(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        return *self == Direction::Left || *self == Direction::Right;
    }

    pub fn is_vertical(&self) -> bool {
        return *self == Direction::Up || *self == Direction::Down;
    }
}
