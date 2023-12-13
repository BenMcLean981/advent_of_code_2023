use std::str::FromStr;

use super::sequence::Sequence;

#[derive(Debug, PartialEq, Eq)]
pub struct SpringRow {
    springs: Vec<SpringType>,
}

impl SpringRow {
    pub fn new(springs: Vec<SpringType>) -> Self {
        return SpringRow { springs };
    }

    pub fn get_sequence(&self) -> Sequence {
        let before_unknown = self
            .springs
            .split(|s| *s == SpringType::Unknown)
            .nth(0)
            .unwrap();

        let lengths = before_unknown
            .split(|s| *s == SpringType::Operational)
            .filter(|s| !s.is_empty())
            .map(|s| s.len())
            .collect::<Vec<usize>>();

        let last = before_unknown.last();
        let is_last_unclear =
            last.is_none() || *last.unwrap() == SpringType::Damaged;

        if lengths.len() > 0 && is_last_unclear {
            return Sequence::new(lengths[0..lengths.len() - 1].to_vec());
        } else {
            return Sequence::new(lengths);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseSpringRowError;

impl FromStr for SpringRow {
    type Err = ParseSpringRowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let springs =
            s.chars().map(SpringType::from).collect::<Vec<SpringType>>();

        return Ok(SpringRow::new(springs));
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SpringType {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for SpringType {
    fn from(value: char) -> Self {
        return match value {
            '?' => SpringType::Unknown,
            '.' => SpringType::Operational,
            '#' => SpringType::Damaged,
            _ => panic!(),
        };
    }
}
