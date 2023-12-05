pub struct Range {
    pub lower: u32,
    pub upper: u32,
}

impl Range {
    pub fn new(lower: u32, size: u32) -> Self {
        return Range {
            lower,
            upper: lower + size,
        };
    }

    pub fn contains(&self, n: u32) -> bool {
        return n >= self.lower && n <= self.upper;
    }

    pub fn len(&self) -> u32 {
        return self.upper - self.lower;
    }
}
