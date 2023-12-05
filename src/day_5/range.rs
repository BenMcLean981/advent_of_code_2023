#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
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

    pub fn intersection(r1: Range, r2: Range) -> Option<Range> {
        if r1.is_superset(r2) {
            return Some(r2.clone());
        } else if r2.is_superset(r1) {
            return Some(r1.clone());
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

    fn is_superset(&self, r: Range) -> bool {
        return self.contains(r.lower) && self.contains(r.upper);
    }

    pub fn subtract_all(base: Range, subs: Vec<Range>) -> Vec<Range> {
        let mut results = vec![base];
        let mut next_results = vec![];

        subs.iter().for_each(|sub| {
            next_results = results
                .iter()
                .flat_map(|r| Range::difference(*r, *sub))
                .collect();
            results = next_results.clone();
            next_results = vec![];
        });

        return results;
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

    pub fn contains(&self, n: u64) -> bool {
        return n >= self.lower && n <= self.upper;
    }

    pub fn len(&self) -> u64 {
        return self.upper - self.lower;
    }
}
