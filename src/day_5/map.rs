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
        let intersections = self
            .maps
            .iter()
            .map(|m| Range::intersection(range, m.source))
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .collect::<Vec<Range>>();

        let mut results = Range::subtract_all(range, intersections.clone());
        let mut mapped: Vec<Range> = intersections
            .iter()
            .map(|r| self.find_map(*r).map_range(*r))
            .collect();

        results.append(&mut mapped);

        return results;
    }

    fn find_map(&self, r: Range) -> &RangeMap {
        return self
            .maps
            .iter()
            .find(|m| Range::intersection(r, m.source).is_some())
            .unwrap();
    }
}
