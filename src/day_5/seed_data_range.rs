use super::range::Range;

pub struct SeedDataRange {
    pub seed: Range,
    pub soil: Vec<Range>,
    pub fertilizer: Vec<Range>,
    pub water: Vec<Range>,
    pub light: Vec<Range>,
    pub temperature: Vec<Range>,
    pub humidity: Vec<Range>,
    pub location: Vec<Range>,
}
