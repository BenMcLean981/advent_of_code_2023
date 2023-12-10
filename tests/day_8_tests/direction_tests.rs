use advent_of_code_2023::day_8::direction::Direction;

#[test]
pub fn from_left_makes_direction() {
    assert_eq!(Direction::Left, Direction::from('L'));
}

#[test]
pub fn from_right_makes_direction() {
    assert_eq!(Direction::Right, Direction::from('R'));
}
