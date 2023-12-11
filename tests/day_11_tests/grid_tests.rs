use advent_of_code_2023::day_11::grid::Grid;

#[test]
pub fn from_lines_makes_grids() {
    let actual = Grid::from_lines(vec![
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

    let expected = Grid::new(vec![
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
