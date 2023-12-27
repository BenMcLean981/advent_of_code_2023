use super::signal::Signal;

#[derive(Debug, Clone)]
pub struct Message {
    pub sender: String,
    pub target: String,
    pub signal: Signal,
}

impl Message {
    pub fn new(sender: String, target: String, signal: Signal) -> Self {
        return Message {
            sender,
            target,
            signal,
        };
    }
}
