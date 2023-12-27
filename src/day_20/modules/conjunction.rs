use std::collections::{HashMap, HashSet, VecDeque};

use crate::day_20::{message::Message, signal::Signal};

use super::module::Module;

pub struct Conjunction {
    name: String,
    memory: HashMap<String, Signal>,
    targets: Vec<String>,
}

impl Conjunction {
    pub fn new(name: String, targets: Vec<String>) -> Self {
        return Conjunction {
            name,
            memory: HashMap::new(),
            targets,
        };
    }

    fn update(&mut self, message: &Message) {
        self.memory.insert(message.sender.clone(), message.signal);
    }

    fn get_all(&self) -> Signal {
        let values = self.memory.values().copied().collect::<HashSet<Signal>>();

        if values.len() == 1 && values.contains(&Signal::High) {
            return Signal::Low;
        } else {
            return Signal::High;
        }
    }
}

impl Module for Conjunction {
    fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn get_targets(&self) -> Vec<String> {
        return self.targets.clone();
    }

    fn add_input(&mut self, input: String) {
        self.memory.insert(input, Signal::Low);
    }

    fn process(&mut self, message: &Message) -> VecDeque<Message> {
        self.update(message);

        let output = self.get_all();

        return self
            .targets
            .iter()
            .map(|t| Message::new(self.name.clone(), t.clone(), output))
            .collect();
    }
}
