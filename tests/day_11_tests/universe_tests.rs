use advent_of_code_2023::{
    day_10::position::Position,
    day_11::universe::{transpose, Universe},
};

#[test]
pub fn from_lines_makes_grids() {
    let actual = Universe::from_lines(vec![
        "...#.......",
        "........#..",
        "#..........",
        "...........",
        ".......#...",
        ".#.........",
        "..........#",
        "...........",
        "........#..",
        "#...#......",
    ]);

    let expected = Universe::new(vec![
        vec![
            false, false, false, true, false, false, false, false, false,
            false, false,
        ],
        vec![
            false, false, false, false, false, false, false, false, true,
            false, false,
        ],
        vec![
            true, false, false, false, false, false, false, false, false,
            false, false,
        ],
        vec![
            false, false, false, false, false, false, false, false, false,
            false, false,
        ],
        vec![
            false, false, false, false, false, false, false, true, false,
            false, false,
        ],
        vec![
            false, true, false, false, false, false, false, false, false,
            false, false,
        ],
        vec![
            false, false, false, false, false, false, false, false, false,
            false, true,
        ],
        vec![
            false, false, false, false, false, false, false, false, false,
            false, false,
        ],
        vec![
            false, false, false, false, false, false, false, false, true,
            false, false,
        ],
        vec![
            true, false, false, false, true, false, false, false, false, false,
            false,
        ],
    ]);

    assert_eq!(expected, actual);
}

// #[test]
// pub fn get_sum_distances() {
//     let universe = Universe::from_lines(vec![
//         "...#......",
//         ".......#..",
//         "#.........",
//         "..........",
//         "......#...",
//         ".#........",
//         ".........#",
//         "..........",
//         ".......#..",
//         "#...#.....",
//     ]);

//     assert_eq!(374, universe.get_sum_distances(1));
// }

// #[test]
// pub fn get_sum_square_case() {
//     let universe = Universe::from_lines(vec!["#..", "...", "..#"]);

//     assert_eq!(4, universe.get_sum_distances(0));
//     assert_eq!(6, universe.get_sum_distances(1));
//     assert_eq!(8, universe.get_sum_distances(2));
// }

// #[test]
// pub fn get_sum_col_case() {
//     let universe = Universe::from_lines(vec!["#..", "..#"]);

//     assert_eq!(3, universe.get_sum_distances(0));
//     assert_eq!(4, universe.get_sum_distances(1));
//     assert_eq!(5, universe.get_sum_distances(2));
// }

// #[test]
// pub fn get_sum_big_square_case() {
//     let universe = Universe::from_lines(vec!["#...", "....", "....", "...#"]);

//     assert_eq!(6, universe.get_sum_distances(0));
//     assert_eq!(10, universe.get_sum_distances(1));
//     assert_eq!(14, universe.get_sum_distances(2));
// }

#[test]
pub fn get_sum_distances_extra_expansion() {
    let universe = Universe::from_lines(vec![
        "...#......",
        ".......#..",
        "#.........",
        "..........",
        "......#...",
        ".#........",
        ".........#",
        "..........",
        ".......#..",
        "#...#.....",
    ]);

    assert_eq!(8410, universe.get_sum_distances(100));
}

#[test]
pub fn transpose_makes_matrix_tranposition() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];

    let actual = transpose(&matrix);
    let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];

    assert_eq!(expected, actual);
}
