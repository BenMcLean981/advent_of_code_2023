use std::collections::HashMap;

use super::{cube::Cube, viewing::Viewing};

#[derive(Debug, PartialEq, Clone)]
pub struct Game {
    pub totals: HashMap<Cube, u32>,
    pub viewings: Vec<Viewing>,
}

impl Game {
    pub fn new(totals: &HashMap<Cube, u32>, viewings: &Vec<Viewing>) -> Self {
        return Game {
            totals: totals.clone(),
            viewings: viewings.clone(),
        };
    }
}
