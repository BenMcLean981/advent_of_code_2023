use std::{collections::HashMap, str::FromStr};

use super::sequence::Sequence;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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

    pub fn len(&self) -> usize {
        return self.springs.len();
    }

    pub fn skip_first(&self) -> SpringRow {
        return self.skip(1);
    }

    pub fn skip(&self, num: usize) -> SpringRow {
        return SpringRow::new(
            self.springs.iter().skip(num).map(|s| s.clone()).collect(),
        );
    }

    pub fn set_first(&self, spring: SpringType) -> SpringRow {
        let mut springs = vec![spring];

        springs.extend(self.springs.iter().skip(1).map(|s| s.clone()));

        return SpringRow::new(springs);
    }
}

pub fn count_possible_rows(
    row: &SpringRow,
    sequence: &Sequence,
    memo: &mut HashMap<(SpringRow, Sequence), usize>,
) -> usize {
    let key = (row.clone(), sequence.clone());

    if memo.contains_key(&key) {
        return *memo.get(&key).unwrap();
    } else {
        let result = force_count_possible_rows(row, sequence, memo);

        memo.insert(key, result);

        return result;
    }
}

fn force_count_possible_rows(
    row: &SpringRow,
    sequence: &Sequence,
    memo: &mut HashMap<(SpringRow, Sequence), usize>,
) -> usize {
    // had to find this online, too complicated for my brain to find
    // the recursive solution. Spent 2-3 days on this.

    if row.len() == 0 && sequence.len() == 0 {
        return 1;
    } else if row.len() == 0 && sequence.len() != 0 {
        return 0;
    } else if sequence.len() == 0 {
        if row.springs.iter().any(|s| *s == SpringType::Damaged) {
            return 0;
        } else {
            return 1;
        }
    }

    let too_small = row.len() < sequence.sum() + sequence.len() - 1;

    if too_small {
        return 0;
    }

    let first = *row.springs.first().unwrap();
    if first == SpringType::Operational {
        return count_possible_rows(&row.skip_first(), sequence, memo);
    }

    if first == SpringType::Damaged {
        let count = sequence.first().unwrap();
        let remaining = sequence.skip_first();

        let not_enough_damaged_possible = row.springs[0..count]
            .iter()
            .any(|s| *s == SpringType::Operational);
        let too_many_damaged = row.springs.len() > count
            && row.springs[count] == SpringType::Damaged;

        if not_enough_damaged_possible || too_many_damaged {
            return 0;
        }

        return count_possible_rows(&row.skip(count + 1), &remaining, memo);
    }

    return count_possible_rows(
        &row.set_first(SpringType::Damaged),
        sequence,
        memo,
    ) + count_possible_rows(
        &row.set_first(SpringType::Operational),
        sequence,
        memo,
    );
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
