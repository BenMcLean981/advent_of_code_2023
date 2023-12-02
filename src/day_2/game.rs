use super::{cube_count::CubeCount, viewing::Viewing};

#[derive(Debug, PartialEq, Clone)]
pub struct Game {
    pub total: Vec<CubeCount>,
    pub viewings: Vec<Viewing>,
}
