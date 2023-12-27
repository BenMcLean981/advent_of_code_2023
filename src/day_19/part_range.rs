use super::range::Range;

#[derive(Debug, Clone, Copy)]
pub struct PartRange {
    pub x: Range,
    pub m: Range,
    pub a: Range,
    pub s: Range,
}

impl PartRange {
    pub fn full() -> Self {
        return PartRange {
            x: Range::new(1, 4000),
            m: Range::new(1, 4000),
            a: Range::new(1, 4000),
            s: Range::new(1, 4000),
        };
    }

    pub fn count(&self) -> u32 {
        return self.x.size() & self.m.size() & self.a.size() & self.s.size();
    }
}
