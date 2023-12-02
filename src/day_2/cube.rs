use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Cube {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCubeError;

impl FromStr for Cube {
    type Err = ParseCubeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "red" => Ok(Cube::Red),
            "green" => Ok(Cube::Green),
            "blue" => Ok(Cube::Blue),
            _ => Err(ParseCubeError),
        }
    }
}
