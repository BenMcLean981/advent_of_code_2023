#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    lower: u32,
    upper: u32,
}

impl Range {
    pub fn new(lower: u32, upper: u32) -> Self {
        return Range { lower, upper };
    }

    pub fn intersection(r1: Range, r2: Range) -> Option<Self> {
        if r1.contains(r2.lower) && r1.contains(r2.upper) {
            return Some(r2);
        } else if r2.contains(r1.lower) && r2.contains(r1.upper) {
            return Some(r1);
        } else if r1.contains(r2.upper) && r2.contains(r1.lower) {
            return Some(Range::new(r1.lower, r2.upper));
        } else if r2.contains(r1.upper) && r1.contains(r2.lower) {
            return Some(Range::new(r2.lower, r1.upper));
        } else {
            return None;
        }
    }

    pub fn difference(base: Range, sub: Range) -> Vec<Range> {
        let intersection = Range::intersection(base, sub);

        if let Some(intersection) = intersection {
            if intersection == base {
                return vec![];
            } else if intersection.upper == base.upper {
                return vec![Range {
                    lower: base.lower,
                    upper: intersection.lower - 1,
                }];
            } else if intersection.lower == base.lower {
                return vec![Range {
                    lower: intersection.upper + 1,
                    upper: base.upper,
                }];
            } else {
                return vec![
                    Range {
                        lower: base.lower,
                        upper: intersection.lower - 1,
                    },
                    Range {
                        lower: intersection.upper + 1,
                        upper: base.upper,
                    },
                ];
            }
        } else {
            return vec![base];
        }
    }

    pub fn contains(&self, n: u32) -> bool {
        return self.lower <= n && n <= self.upper;
    }

    pub fn size(&self) -> u32 {
        return self.upper - self.lower;
    }
}
