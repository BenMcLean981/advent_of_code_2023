#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Signal {
    Low,
    High,
}

impl Signal {
    pub fn toggle(&self) -> Self {
        match self {
            Signal::Low => Signal::High,
            Signal::High => Signal::Low,
        }
    }
}
