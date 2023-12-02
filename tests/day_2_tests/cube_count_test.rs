use advent_of_code_2023::day_2::{cube::Cube, cube_count::CubeCount};

#[test]
pub fn new_cube_count_makes_new() {
    let cube = Cube::Green;
    let count = 123;

    let actual = CubeCount::new(cube, count);

    assert_eq!(cube, actual.cube);
    assert_eq!(count, actual.count);
}

