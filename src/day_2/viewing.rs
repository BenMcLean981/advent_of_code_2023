use std::collections::HashMap;

use super::cube::Cube;

#[derive(Debug, PartialEq, Clone)]
pub struct Viewing {
    pub counts: HashMap<Cube, u32>,
}

impl Viewing {
    pub fn new(counts: &HashMap<Cube, u32>) -> Viewing {
        return Viewing {
            counts: counts.clone(),
        };
    }
}
