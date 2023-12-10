#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TileType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl From<char> for TileType {
    fn from(value: char) -> Self {
        return match value {
            '|' => TileType::Vertical,
            '-' => TileType::Horizontal,
            'L' => TileType::NorthEast,
            'J' => TileType::NorthWest,
            '7' => TileType::SouthWest,
            'F' => TileType::SouthEast,
            '.' => TileType::Ground,
            'S' => TileType::Start,
            _ => panic!(),
        };
    }
}
