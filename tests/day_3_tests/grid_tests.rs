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

    let actual = grid.get_part_numbers();
    let expected = vec![123, 12, 67, 5];

    assert_eq!(true, have_same_items(expected, actual));
}

#[test]
pub fn gear_gear_ratios_gets_all_gear_ratios() {
    let grid = Grid::from_lines(vec![
        "467..114..".to_string(),
        "...*......".to_string(),
        "..35..633.".to_string(),
        "......#...".to_string(),
        "617*......".to_string(),
        ".....+.58.".to_string(),
        "..592.....".to_string(),
        "......755.".to_string(),
        "...$.*....".to_string(),
        ".664.598..".to_string(),
    ]);

    let actual = grid.get_gear_ratios();
    let expected = vec![16345, 451490];

    assert_eq!(true, have_same_items(expected, actual));
}

fn have_same_items(s1: Vec<u32>, s2: Vec<u32>) -> bool {
    let sorted_1 = sort(s1);
    let sorted_2 = sort(s2);

    return sorted_1.eq(&sorted_2);
}

fn sort(vec: Vec<u32>) -> Vec<u32> {
    let mut result = vec.to_vec();

    result.sort();

    return result;
}
