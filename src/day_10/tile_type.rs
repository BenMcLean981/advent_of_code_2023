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

impl TileType {
    pub fn get_connections(&self) -> Vec<Connection> {
        return match self {
            TileType::Vertical => vec![Connection::North, Connection::South],
            TileType::Horizontal => vec![Connection::East, Connection::West],
            TileType::NorthEast => vec![Connection::North, Connection::East],
            TileType::NorthWest => vec![Connection::North, Connection::West],
            TileType::SouthWest => vec![Connection::South, Connection::West],
            TileType::SouthEast => vec![Connection::South, Connection::East],
            TileType::Start => vec![
                Connection::North,
                Connection::East,
                Connection::South,
                Connection::West,
            ],
            TileType::Ground => panic!(),
        };
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Connection {
    North,
    East,
    South,
    West,
}
