use super::instruction::{Instruction, Operation};

pub struct Boxes {
    boxes: Vec<Box>,
}

impl Boxes {
    pub fn new() -> Self {
        let mut boxes: Vec<Box> = vec![];

        for _ in 0..256 {
            boxes.push(Box::new());
        }

        return Boxes { boxes };
    }

    pub fn apply(&mut self, instruction: &Instruction) {
        let index = super::hash::hash(&instruction.label) as usize;

        match instruction.operation {
            Operation::Remove => self.remove(index, &instruction.label),
            Operation::Insert(focal_length) => {
                self.add(index, Lens::new(&instruction.label, focal_length))
            }
        }
    }

    fn remove(&mut self, index: usize, label: &str) {
        self.boxes[index].remove(&label);
    }

    fn add(&mut self, index: usize, lens: Lens) {
        self.boxes[index].add(lens);
    }

    pub fn compute_focusing_power(&self) -> u32 {
        return self
            .boxes
            .iter()
            .enumerate()
            .map(|(i, b)| (i as u32 + 1) * b.compute_focusing_power())
            .sum();
    }
}

struct Box {
    lenses: Vec<Lens>,
}

impl Box {
    pub fn new() -> Self {
        return Box { lenses: vec![] };
    }

    pub fn remove(&mut self, label: &str) {
        self.lenses = self
            .lenses
            .iter()
            .filter(|l| l.label != label)
            .cloned()
            .collect::<Vec<Lens>>();
    }

    pub fn add(&mut self, lens: Lens) {
        // replace existing
        for (i, l) in self.lenses.iter().enumerate() {
            if l.label == lens.label {
                self.lenses[i] = lens;
                return;
            }
        }

        // otherwise add to back.
        self.lenses.push(lens);
    }

    pub fn compute_focusing_power(&self) -> u32 {
        let result = self
            .lenses
            .iter()
            .enumerate()
            .map(|(i, l)| (i as u32 + 1) * l.focal_length)
            .sum();

        return result;
    }
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: u32,
}

impl Lens {
    pub fn new(label: &str, focal_length: u32) -> Self {
        return Lens {
            label: label.to_string(),
            focal_length,
        };
    }
}
