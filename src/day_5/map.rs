use super::{
    range::Range,
    range_map::{self, RangeMap},
};

pub struct Map {
    maps: Vec<RangeMap>,
}

impl Map {
    pub fn new(maps: Vec<RangeMap>) -> Self {
        // could validate no overlap here, requires Range.intersect

        return Map { maps };
    }

    pub fn map(&self, n: u32) -> u32 {
        let range_map = self.maps.iter().find(|m| m.should_map(n));

        if let Some(range_map) = range_map {
            return range_map.map(n);
        } else {
            return n;
        }
    }
}
