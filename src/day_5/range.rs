#[derive(Debug, PartialEq, Eq)]
pub struct Range {
    pub lower: u32,
    pub upper: u32,
}

impl Range {
    pub fn new(lower: u32, size: u32) -> Self {
        if size == 0 {
            panic!();
        }

        return Range {
            lower,
            upper: lower + size - 1,
        };
    }

    pub fn contains(&self, n: u32) -> bool {
        return n >= self.lower && n <= self.upper;
    }

    pub fn len(&self) -> u32 {
        return self.upper - self.lower;
    }
}
