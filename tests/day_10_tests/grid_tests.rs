use advent_of_code_2023::day_10::{grid::Grid, tile_type::TileType};

#[test]
pub fn from_lines_makes_grid() {
    let expected = Grid::new(vec![
        vec![
            TileType::Ground,
            TileType::Ground,
            TileType::SouthEast,
            TileType::SouthWest,
            TileType::Ground,
        ],
        vec![
            TileType::Ground,
            TileType::SouthEast,
            TileType::NorthWest,
            TileType::Vertical,
            TileType::Ground,
        ],
        vec![
            TileType::Start,
            TileType::NorthWest,
            TileType::Ground,
            TileType::NorthEast,
            TileType::SouthWest,
        ],
        vec![
            TileType::Vertical,
            TileType::SouthEast,
            TileType::Horizontal,
            TileType::Horizontal,
            TileType::NorthWest,
        ],
        vec![
            TileType::NorthEast,
            TileType::NorthWest,
            TileType::Ground,
            TileType::Ground,
            TileType::Ground,
        ],
    ]);

    let actual =
        Grid::from_lines(vec!["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."]);

    assert_eq!(expected, actual);
}
