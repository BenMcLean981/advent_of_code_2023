use super::cube_count::CubeCount;

#[derive(Debug, PartialEq, Clone)]
pub struct Viewing {
    pub counts: Vec<CubeCount>,
}

impl Viewing {
    pub fn new(counts: &Vec<CubeCount>) -> Viewing {
        return Viewing {
            counts: counts.clone(),
        };
    }
}
