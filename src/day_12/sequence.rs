use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Sequence {
    sequence: Vec<usize>,
    next_unclear: bool,
}

impl Sequence {
    pub fn make_clear(sequence: Vec<usize>) -> Self {
        return Sequence {
            sequence,
            next_unclear: false,
        };
    }

    pub fn make_unclear(sequence: Vec<usize>) -> Self {
        return Sequence {
            sequence,
            next_unclear: true,
        };
    }

    pub fn is_partial(&self, sequence: &Sequence) -> bool {
        if self.sequence.len() > sequence.sequence.len() {
            return false;
        }

        if self.sequence.len() == 0 {
            return true;
        } else if self.next_unclear {
            let num_eq: usize = self.sequence.len() - 1;

            let sub = sequence.sequence[0..num_eq].to_vec();
            let self_sub = self.sequence[0..num_eq].to_vec();

            let next = sequence.sequence.iter().skip(num_eq).nth(0).unwrap();
            let self_next = self.sequence.iter().skip(num_eq).nth(0).unwrap();

            return self_sub == sub && self_next <= next;
        } else {
            let sub = sequence.sequence[0..self.sequence.len()].to_vec();

            return self.sequence == sub;
        }
    }

    pub fn expand(&self, count: usize) -> Sequence {
        let mut result: Vec<usize> = self.sequence.clone();

        for _ in 0..count - 1 {
            result.extend(self.sequence.clone())
        }

        return Sequence::make_clear(result);
    }

    pub fn len(&self) -> usize {
        return self.sequence.len();
    }

    pub fn sum(&self) -> usize {
        return self.sequence.iter().sum();
    }

    pub fn first(&self) -> Option<usize> {
        let first = self.sequence.first();

        if first.is_none() {
            return None;
        } else {
            return Some(*first.unwrap());
        }
    }

    pub fn skip_first(&self) -> Sequence {
        return Sequence::make_clear(
            self.sequence.iter().skip(1).map(|s| s.clone()).collect(),
        );
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseSequenceError;

impl FromStr for Sequence {
    type Err = ParseSequenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| usize::from_str(s).unwrap())
            .collect::<Vec<usize>>();

        return Ok(Sequence::make_clear(nums));
    }
}
