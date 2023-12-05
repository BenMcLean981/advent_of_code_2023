use super::range::Range;

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
