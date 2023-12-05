use std::str::FromStr;

use super::{range::Range, range_map::RangeMap};

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    maps: Vec<RangeMap>,
}

impl Map {
    pub fn new(maps: Vec<RangeMap>) -> Self {
        // could validate no overlap here, requires Range.intersect

        return Map { maps };
    }

    pub fn from_lines(lines: &Vec<&str>) -> Self {
        let maps = lines[1..]
            .iter()
            .map(|l| RangeMap::from_str(*l).unwrap())
            .collect();

        return Map::new(maps);
    }

    pub fn map(&self, n: u64) -> u64 {
        let range_map = self.maps.iter().find(|m| m.should_map(n));

        if let Some(range_map) = range_map {
            return range_map.map(n);
        } else {
            return n;
        }
    }

    pub fn map_range(&self, range: Range) -> Vec<Range> {
        return vec![];
    }
}
