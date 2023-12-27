use super::{part_range::PartRange, rule::Destination};

#[derive(Debug, Clone)]
pub struct PartRangeMap {
    pub range: PartRange,
    pub destination: Destination,
}

impl PartRangeMap {
    pub fn new(range: PartRange, destination: Destination) -> Self {
        return PartRangeMap { range, destination };
    }
}
