use advent_of_code_2023::day_2::{cube::Cube, cube_count::CubeCount, viewing::Viewing};

#[test]
pub fn new_cube_count_makes_new() {
    let counts = vec![
        CubeCount::new(Cube::Green, 5),
        CubeCount::new(Cube::Blue, 10),
    ];

    let actual = Viewing::new(&counts);

    assert_eq!(counts, actual.counts);
}
