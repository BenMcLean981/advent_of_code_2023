#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(),
        }
    }
}
