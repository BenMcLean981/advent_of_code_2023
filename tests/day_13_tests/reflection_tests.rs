use advent_of_code_2023::day_13::reflection::{Item, Mirror, Reflection};

#[test]
pub fn from_lines_makes_reflection() {
    let lines = vec!["#.##", "#...", ".#..", "..##"];

    let actual = Reflection::from_lines(lines);
    let expected = Reflection::new(vec![
        vec![Item::Rock, Item::Ash, Item::Rock, Item::Rock],
        vec![Item::Rock, Item::Ash, Item::Ash, Item::Ash],
        vec![Item::Ash, Item::Rock, Item::Ash, Item::Ash],
        vec![Item::Ash, Item::Ash, Item::Rock, Item::Rock],
    ]);

    assert_eq!(expected, actual);
}

#[test]
pub fn find_mirrored_col_gets_index() {
    let reflection = Reflection::from_lines(vec![
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
    ]);

    assert_eq!(Some(5), reflection.find_mirrored_col());
}

#[test]
pub fn find_mirrored_col_gets_rows_mirroed_returns_none() {
    let reflection = Reflection::from_lines(vec![
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ]);

    assert_eq!(None, reflection.find_mirrored_col());
}

#[test]
pub fn is_mirror_after_col_not_after_col_returns_false() {
    let reflection = Reflection::from_lines(vec![
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
    ]);

    assert!(!reflection.is_mirror_after_col(3));
    assert!(!reflection.is_mirror_after_col(4));
    assert!(!reflection.is_mirror_after_col(6));
}

#[test]
pub fn is_mirror_after_col_after_col_returns_true() {
    let reflection = Reflection::from_lines(vec![
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
    ]);

    assert!(reflection.is_mirror_after_col(5));
}

#[test]
pub fn are_cols_mirrored_not_mirrored_returns_false() {
    let reflection = Reflection::from_lines(vec![
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
    ]);

    assert!(!reflection.are_cols_mirroed(4, 8));
}

#[test]
pub fn are_cols_mirrored_mirrored_returns_true() {
    let reflection = Reflection::from_lines(vec![
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
    ]);

    assert!(reflection.are_cols_mirroed(4, 7));
}

#[test]
pub fn find_mirrored_row_gets_index() {
    let reflection = Reflection::from_lines(vec![
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ]);

    assert_eq!(Some(4), reflection.find_mirrored_row());
}

#[test]
pub fn find_mirrored_row_gets_rows_mirroed_returns_none() {
    let reflection = Reflection::from_lines(vec![
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
    ]);

    assert_eq!(None, reflection.find_mirrored_row());
}

#[test]
pub fn is_mirror_after_row_not_after_row_returns_false() {
    let reflection = Reflection::from_lines(vec![
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ]);

    assert!(!reflection.is_mirror_after_row(3));
    assert!(!reflection.is_mirror_after_row(5));
    assert!(!reflection.is_mirror_after_row(6));
}

#[test]
pub fn is_mirror_after_row_after_row_returns_true() {
    let reflection = Reflection::from_lines(vec![
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ]);

    assert!(reflection.is_mirror_after_row(4));
}

#[test]
pub fn are_rows_mirrored_not_mirrored_returns_false() {
    let reflection = Reflection::from_lines(vec![
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ]);

    assert!(!reflection.are_rows_mirroed(1, 3));
}

#[test]
pub fn are_rows_mirrored_mirrored_returns_true() {
    let reflection = Reflection::from_lines(vec![
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ]);

    assert!(reflection.are_rows_mirroed(2, 7));
}

#[test]
pub fn find_mirror_finds_row() {
    let reflection = Reflection::from_lines(vec![
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ]);

    assert_eq!(Mirror::Horizontal(4), reflection.find_mirror());
}

#[test]
pub fn find_mirror_finds_col() {
    let reflection = Reflection::from_lines(vec![
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
    ]);

    assert_eq!(Mirror::Vertical(5), reflection.find_mirror());
}
