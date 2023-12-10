use advent_of_code_2023::day_10::{
    grid::Grid, position::Position, tile_type::TileType,
};

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

#[test]
pub fn get_at_position() {
    let grid = Grid::new(vec![
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

    assert_eq!(
        TileType::SouthEast,
        grid.get_at_position(&Position::new(3, 1))
    )
}

#[test]
pub fn get_available_positions() {
    let grid = Grid::new(vec![
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

    let expected = vec![Position::new(4, 1), Position::new(3, 2)];
    let actual = grid.get_available_positions(Position::new(3, 1));

    assert_eq!(true, have_same_items(expected, actual));

    let expected = vec![Position::new(2, 1), Position::new(3, 0)];
    let actual = grid.get_available_positions(Position::new(2, 0));

    assert_eq!(true, have_same_items(expected, actual));
}

fn have_same_items(s1: Vec<Position>, s2: Vec<Position>) -> bool {
    let sorted_1 = sort(s1);
    let sorted_2 = sort(s2);

    return sorted_1.eq(&sorted_2);
}

fn sort(vec: Vec<Position>) -> Vec<Position> {
    let mut result = vec.to_vec();

    result.sort();

    return result;
}
