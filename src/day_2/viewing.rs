use std::{collections::HashMap, str::FromStr};

use super::cube::Cube;

#[derive(Debug, PartialEq, Clone)]
pub struct Viewing {
    pub counts: HashMap<Cube, u32>,
}

impl Viewing {
    pub fn new(counts: &HashMap<Cube, u32>) -> Self {
        return Viewing {
            counts: counts.clone(),
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseViewingError;

impl FromStr for Viewing {
    type Err = ParseViewingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = s
            .split(", ")
            .map(parse_entry)
            .collect::<HashMap<Cube, u32>>();

        return Ok(Viewing::new(&result));
    }
}

fn parse_entry(entry: &str) -> (Cube, u32) {
    let split = entry.trim().split(' ').collect::<Vec<&str>>();

    let count = u32::from_str(split[0]).unwrap();
    let cube = Cube::from_str(split[1]).unwrap();

    return (cube, count);
}
