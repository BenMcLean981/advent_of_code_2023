use std::str::FromStr;

use super::range::Range;

#[derive(Debug, PartialEq, Eq)]
pub struct RangeMap {
    source: Range,
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

    pub fn should_map(&self, n: u32) -> bool {
        return self.source.contains(n);
    }

    pub fn map(&self, n: u32) -> u32 {
        if self.should_map(n) {
            let diff = n - self.source.lower;

            return self.destination.lower + diff;
        } else {
            return n;
        }
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
            .map(|s| u32::from_str(s).unwrap())
            .collect::<Vec<u32>>();

        let destination_start = split[0];
        let source_start = split[1];
        let size = split[2];

        let destination = Range::new(destination_start, size);
        let source = Range::new(source_start, size);

        return Ok(RangeMap::new(source, destination));
    }
}
