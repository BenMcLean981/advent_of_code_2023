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

#[test]
pub fn expand_adds_rows_and_cols() {
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

    let actual = universe.expand();
    let expected = Universe::from_lines(vec![
        "....#........",
        ".........#...",
        "#............",
        ".............",
        ".............",
        "........#....",
        ".#...........",
        "............#",
        ".............",
        ".............",
        ".........#...",
        "#....#.......",
    ]);

    assert_eq!(expected, actual);
}

#[test]
pub fn get_sum_distances() {
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

    assert_eq!(374, universe.get_sum_distances(1));
}

#[test]
pub fn position_get_manhattan_distance() {
    let p1 = Position::new(6, 1);
    let p2 = Position::new(11, 5);

    assert_eq!(9, p1.get_manhattan_distance(&p2));
}

#[test]
pub fn transpose_makes_matrix_tranposition() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];

    let actual = transpose(&matrix);
    let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];

    assert_eq!(expected, actual);
}
