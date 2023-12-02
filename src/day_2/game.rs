use std::collections::HashMap;

use super::{cube::Cube, viewing::Viewing};

#[derive(Debug, PartialEq, Clone)]
pub struct Game {
    pub total: HashMap<Cube, u32>,
    pub viewings: Vec<Viewing>,
}
