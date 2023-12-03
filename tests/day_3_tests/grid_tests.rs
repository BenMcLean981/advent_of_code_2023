use std::collections::HashSet;

use advent_of_code_2023::day_3::grid::Grid;

#[test]
pub fn grid_get_part_numbers_just_dots_returns_empty() {
    let grid = Grid::from_lines(vec![
        "........".to_string(),
        "........".to_string(),
        "........".to_string(),
        "........".to_string(),
        "........".to_string(),
        "........".to_string(),
        "........".to_string(),
    ]);

    let actual = grid.get_part_numbers();
    let expected = HashSet::<u32>::new();

    assert_eq!(expected, actual);
}

#[test]
pub fn grid_get_part_numbers_one_part_number_returns_number() {
    let grid = Grid::from_lines(vec![
        "........".to_string(),
        ".123....".to_string(),
        "....#...".to_string(),
        "........".to_string(),
        "........".to_string(),
        "........".to_string(),
        "........".to_string(),
    ]);

    let actual = grid.get_part_numbers();
    let expected: HashSet<u32> = HashSet::from_iter(vec![123]);

    assert_eq!(expected, actual);
}
