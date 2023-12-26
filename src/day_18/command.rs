use std::str::FromStr;

use super::xy::Xy;

pub struct Command {
    direction_1: Direction,
    distance_1: u64,
    direction_2: Direction,
    distance_2: u64,
}

impl Command {
    pub fn to_vector_1(&self) -> Xy {
        let distance = self.distance_1 as i64;

        match self.direction_1 {
            Direction::Up => Xy::new(0, distance),
            Direction::Right => Xy::new(distance, 0),
            Direction::Down => Xy::new(0, -distance),
            Direction::Left => Xy::new(-distance, 0),
        }
    }

    pub fn to_vector_2(&self) -> Xy {
        let distance_2 = self.distance_2 as i64;

        match self.direction_2 {
            Direction::Up => Xy::new(0, distance_2),
            Direction::Right => Xy::new(distance_2, 0),
            Direction::Down => Xy::new(0, -distance_2),
            Direction::Left => Xy::new(-distance_2, 0),
        }
    }
}

#[derive(Debug)]
pub struct CommandParseError;

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace('(', "").replace(')', "").replace('#', "");
        let s = s.trim();

        let split = s
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        let direction_1 = Direction::from(split[0].chars().nth(0).unwrap());
        let distance_1 = u64::from_str(split[1]).unwrap();

        let radix = 16;
        let distance_2 = u64::from_str_radix(&split[2][0..5], radix).unwrap();
        let direction_2 = u64::from_str_radix(&split[2][5..], radix).unwrap();
        let direction_2 = Direction::from(direction_2);

        return Ok(Command {
            direction_1,
            distance_1,
            direction_2,
            distance_2,
        });
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value.to_ascii_uppercase() {
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            _ => panic!(),
        }
    }
}

impl From<u64> for Direction {
    fn from(value: u64) -> Self {
        match value {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!(),
        }
    }
}
