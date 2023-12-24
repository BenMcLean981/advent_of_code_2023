#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Xy {
    pub x: i32,
    pub y: i32,
}

impl Xy {
    pub fn new(x: i32, y: i32) -> Self {
        return Xy { x, y };
    }

    pub fn add(&self, other: &Xy) -> Self {
        return Xy::new(self.x + other.x, self.y + other.y);
    }
}
