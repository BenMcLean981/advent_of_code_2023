use advent_of_code_2023::day_10::tile_type::{Connection, TileType};

#[test]
pub fn from_char_makes_pipes() {
    assert_eq!(TileType::Vertical, TileType::from('|'));
    assert_eq!(TileType::Horizontal, TileType::from('-'));
    assert_eq!(TileType::NorthEast, TileType::from('L'));
    assert_eq!(TileType::NorthWest, TileType::from('J'));
    assert_eq!(TileType::SouthWest, TileType::from('7'));
    assert_eq!(TileType::SouthEast, TileType::from('F'));
    assert_eq!(TileType::Ground, TileType::from('.'));
    assert_eq!(TileType::Start, TileType::from('S'));
}

#[test]
pub fn get_connections_vertical() {
    assert_eq!(
        vec![Connection::North, Connection::South],
        TileType::Vertical.get_connections()
    );
}

#[test]
pub fn get_connections_horizontal() {
    assert_eq!(
        vec![Connection::East, Connection::West],
        TileType::Horizontal.get_connections()
    );
}

#[test]
pub fn get_connections_north_east() {
    assert_eq!(
        vec![Connection::North, Connection::East],
        TileType::NorthEast.get_connections()
    );
}

#[test]
pub fn get_connections_north_west() {
    assert_eq!(
        vec![Connection::North, Connection::West],
        TileType::NorthWest.get_connections()
    );
}

#[test]
pub fn get_connections_south_east() {
    assert_eq!(
        vec![Connection::South, Connection::East],
        TileType::SouthEast.get_connections()
    );
}

#[test]
pub fn get_connections_south_west() {
    assert_eq!(
        vec![Connection::South, Connection::West],
        TileType::SouthWest.get_connections()
    );
}

#[test]
pub fn get_connections_start() {
    assert_eq!(
        vec![
            Connection::North,
            Connection::East,
            Connection::South,
            Connection::West
        ],
        TileType::Start.get_connections()
    );
}
