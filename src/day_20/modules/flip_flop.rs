use std::collections::VecDeque;

use crate::day_20::{message::Message, signal::Signal};

use super::module::Module;

pub struct FlipFlop {
    name: String,
    state: Signal,
    targets: Vec<String>,
}

impl FlipFlop {
    pub fn new(name: String, targets: Vec<String>) -> Self {
        return FlipFlop {
            name,
            state: Signal::Low,
            targets,
        };
    }

    fn toggle(&mut self) {
        self.state = self.state.toggle();
    }

    fn make_outgoing(&mut self) -> VecDeque<Message> {
        return self
            .targets
            .iter()
            .map(|t| Message::new(self.name.clone(), t.clone(), self.state))
            .collect();
    }
}

impl Module for FlipFlop {
    fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn get_targets(&self) -> Vec<String> {
        return self.targets.clone();
    }

    fn add_input(&mut self, _: String) {
        return;
    }

    fn process(&mut self, message: &Message) -> VecDeque<Message> {
        if message.signal == Signal::Low {
            self.toggle();

            return self.make_outgoing();
        } else {
            return VecDeque::new();
        }
    }
}
