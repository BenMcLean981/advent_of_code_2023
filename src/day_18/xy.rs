#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Xy {
    pub x: i64,
    pub y: i64,
}

impl Xy {
    pub fn new(x: i64, y: i64) -> Self {
        return Xy { x, y };
    }

    pub fn add(&self, other: &Xy) -> Self {
        return Xy::new(self.x + other.x, self.y + other.y);
    }

    pub fn abs_diff(&self, other: &Xy) -> u64 {
        return self.x.abs_diff(other.x) + self.y.abs_diff(other.y);
    }
}
