use std::str::FromStr;

use super::{color::Color, xy::Xy};

pub struct Command {
    direction: Direction,
    distance: usize,
    color: Color,
}

impl Command {
    pub fn to_vector(&self) -> Xy {
        let distance = self.distance as i32;

        match self.direction {
            Direction::Up => Xy::new(0, distance),
            Direction::Right => Xy::new(distance, 0),
            Direction::Down => Xy::new(0, -distance),
            Direction::Left => Xy::new(-distance, 0),
        }
    }
}

#[derive(Debug)]
pub struct CommandParseError;

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace('(', "").replace(')', "");
        let s = s.trim();

        let split = s
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        let direction = Direction::from(split[0].chars().nth(0).unwrap());
        let distance = usize::from_str(split[1]).unwrap();
        let color = Color::from_str(split[2]).unwrap();

        return Ok(Command {
            direction,
            distance,
            color,
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
