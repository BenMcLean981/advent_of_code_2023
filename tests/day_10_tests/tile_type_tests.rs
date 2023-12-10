use advent_of_code_2023::day_10::tile_type::TileType;

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
