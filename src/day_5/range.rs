#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Range {
    pub lower: u64,
    pub upper: u64,
}

impl Range {
    pub fn new(lower: u64, size: u64) -> Self {
        if size == 0 {
            panic!();
        }

        return Range {
            lower,
            upper: lower + size - 1,
        };
    }

    pub fn intersection(r1: &Range, r2: &Range) -> Option<Range> {
        if r1.contains(r2.lower) && r1.contains(r2.upper) {
            return Some(r2.clone());
        } else if r2.contains(r1.lower) && r2.contains(r1.upper) {
            return Some(r1.clone());
        } else if r1.contains(r2.lower) && r2.contains(r1.upper) {
            return Some(Range {
                lower: r2.lower,
                upper: r1.upper,
            });
        } else if r2.contains(r1.lower) && r1.contains(r2.upper) {
            return Some(Range {
                lower: r1.lower,
                upper: r2.upper,
            });
        } else {
            return None;
        }
    }

    pub fn contains(&self, n: u64) -> bool {
        return n >= self.lower && n <= self.upper;
    }

    pub fn len(&self) -> u64 {
        return self.upper - self.lower;
    }
}
