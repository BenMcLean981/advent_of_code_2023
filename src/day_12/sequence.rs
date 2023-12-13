#[derive(Debug, PartialEq, Eq)]
pub struct Sequence {
    sequence: Vec<usize>,
}

impl Sequence {
    pub fn new(sequence: Vec<usize>) -> Self {
        return Sequence { sequence };
    }

    pub fn is_partial(&self, sequence: &Sequence) -> bool {
        if self.sequence.len() > sequence.sequence.len() {
            return false;
        }

        let sub = sequence.sequence[0..self.sequence.len()].to_vec();

        return self.sequence == sub;
    }
}
