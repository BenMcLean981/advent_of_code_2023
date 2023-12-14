use std::str::FromStr;

use super::sequence::Sequence;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SpringRow {
    springs: Vec<SpringType>,
}

impl SpringRow {
    pub fn new(springs: Vec<SpringType>) -> Self {
        return SpringRow { springs };
    }

    pub fn merge(sequences: Vec<&SpringRow>) -> Self {
        let mut springs: Vec<SpringType> = vec![];

        for s in sequences {
            springs.extend(s.springs.clone())
        }

        return SpringRow::new(springs);
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

        if !self.is_done() && lengths.len() > 0 && is_last_unclear {
            return Sequence::make_unclear(lengths[0..lengths.len()].to_vec());
        } else {
            return Sequence::make_clear(lengths);
        }
    }

    pub fn is_done(&self) -> bool {
        return self.springs.iter().all(|s| *s != SpringType::Unknown);
    }

    pub fn fill_next(&self) -> (SpringRow, SpringRow) {
        if self.is_done() {
            panic!();
        }

        return (
            self.replace_first_unknown(SpringType::Operational),
            self.replace_first_unknown(SpringType::Damaged),
        );
    }

    fn replace_first_unknown(&self, v: SpringType) -> SpringRow {
        let mut result: Vec<SpringType> = vec![];

        for (i, s) in self.springs.iter().enumerate() {
            if *s == SpringType::Unknown {
                result.push(v);
                result.extend(self.springs.iter().skip(i + 1).map(|s| *s));
                break;
            } else {
                result.push(*s);
            }
        }

        return SpringRow::new(result);
    }

    pub fn append(&self) -> Self {
        let mut springs = self.springs.clone();
        springs.push(SpringType::Unknown);

        return SpringRow::new(springs);
    }

    pub fn prepend(&self) -> Self {
        let mut springs = vec![SpringType::Unknown];
        springs.extend(self.springs.clone());

        return SpringRow::new(springs);
    }
}

pub fn count_possible_rows(row: &SpringRow, sequence: &Sequence) -> usize {
    return get_possible_rows(row, sequence).len();
}

pub fn get_possible_rows(
    row: &SpringRow,
    sequence: &Sequence,
) -> Vec<SpringRow> {
    let mut pending: Vec<SpringRow> = vec![row.clone()];
    let mut results: Vec<SpringRow> = vec![];

    while !pending.is_empty() {
        let row = pending.pop().unwrap();

        if !row.is_done() {
            let next = row.fill_next();

            if next.0.get_sequence().is_partial(sequence) {
                pending.push(next.0);
            }

            if next.1.get_sequence().is_partial(sequence) {
                pending.push(next.1);
            }
        } else if row.get_sequence() == *sequence {
            results.push(row);
        }
    }

    return results;
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
