use std::str::FromStr;

use super::range::Range;

#[derive(Debug, PartialEq, Eq)]
pub struct RangeMap {
    pub source: Range,
    destination: Range,
}

impl RangeMap {
    pub fn new(source: Range, destination: Range) -> Self {
        if source.len() != destination.len() {
            panic!();
        }

        return RangeMap {
            source,
            destination,
        };
    }

    pub fn should_map(&self, n: u64) -> bool {
        return self.source.contains(n);
    }

    pub fn map(&self, n: u64) -> u64 {
        if self.should_map(n) {
            let diff = n - self.source.lower;

            return self.destination.lower + diff;
        } else {
            return n;
        }
    }

    pub fn map_range(&self, range: Range) -> Range {
        let intersection = Range::intersection(self.source, range).unwrap();
        let is_subset = range == intersection;

        if !is_subset {
            panic!();
        }

        let lower = self.map(range.lower);
        let upper = self.map(range.upper);

        return Range { lower, upper };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRangeMapError;

impl FromStr for RangeMap {
    type Err = ParseRangeMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s
            .split(' ')
            .filter(|s| !s.trim().is_empty())
            .map(|s| u64::from_str(s).unwrap())
            .collect::<Vec<u64>>();

        let destination_start = split[0];
        let source_start = split[1];
        let size = split[2];

        let destination = Range::new(destination_start, size);
        let source = Range::new(source_start, size);

        return Ok(RangeMap::new(source, destination));
    }
}
