use advent_of_code_2023::day_14::platform::{Platform, Rock};

#[test]
pub fn from_lines_makes_platform() {
    let expected = Platform::new(vec![
        vec![Rock::None, Rock::Cube, Rock::Round],
        vec![Rock::Cube, Rock::Cube, Rock::Round],
    ]);

    let actual = Platform::from_lines(vec![".#O", "##O"]);

    assert_eq!(expected, actual);
}

#[test]
pub fn tilt_north_moves_round() {
    let platform = Platform::from_lines(vec![
        "O....#....",
        "O.OO#....#",
        ".....##...",
        "OO.#O....O",
        ".O.....O#.",
        "O.#..O.#.#",
        "..O..#O..O",
        ".......O..",
        "#....###..",
        "#OO..#....",
    ]);

    let expected = Platform::from_lines(vec![
        "OOOO.#.O..",
        "OO..#....#",
        "OO..O##..O",
        "O..#.OO...",
        "........#.",
        "..#....#.#",
        "..O..#.O.O",
        "..O.......",
        "#....###..",
        "#....#....",
    ]);

    assert_eq!(expected, platform.tilt_north());
}

#[test]
pub fn compute_north_load_computes_load() {
    let platform = Platform::from_lines(vec![
        "OOOO.#.O..",
        "OO..#....#",
        "OO..O##..O",
        "O..#.OO...",
        "........#.",
        "..#....#.#",
        "..O..#.O.O",
        "..O.......",
        "#....###..",
        "#....#....",
    ]);

    assert_eq!(136, platform.compute_north_load());
}
