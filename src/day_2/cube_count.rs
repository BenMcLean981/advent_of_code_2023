use super::cube::Cube;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CubeCount {
    pub cube: Cube,
    pub count: u32,
}

impl CubeCount {
    pub fn new(cube: Cube, count: u32) -> CubeCount {
        return CubeCount { cube, count };
    }
}
