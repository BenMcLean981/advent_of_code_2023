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

    assert_eq!(true, actual.is_empty());
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
    let expected = vec![123];

    assert_eq!(expected, actual);
}

#[test]
pub fn grid_get_part_numbers_several_part_number_returns_number() {
    let grid = Grid::from_lines(vec![
        "........".to_string(),
        ".123.12.".to_string(),
        "....#67.".to_string(),
        "....5...".to_string(),
        "........".to_string(),
        "........".to_string(),
        "........".to_string(),
    ]);

    // I need a have_same_items assertion but don't have time right now.
    // if it comes up again I'll do it.
    let actual = grid.get_part_numbers();
    let expected = vec![123, 12, 67, 5];

    assert_eq!(expected, actual);
}
