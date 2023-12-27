use std::collections::VecDeque;

use crate::day_20::{message::Message, signal::Signal};

use super::module::Module;

pub struct Broadcaster {
    targets: Vec<String>,
}

impl Broadcaster {
    pub fn new(targets: Vec<String>) -> Self {
        return Broadcaster { targets };
    }

    pub fn get_name() -> String {
        return "broadcaster".to_string();
    }

    fn make_message(target: String, signal: Signal) -> Message {
        return Message::new(Broadcaster::get_name(), target.clone(), signal);
    }
}

impl Module for Broadcaster {
    fn get_name(&self) -> String {
        return Broadcaster::get_name();
    }

    fn get_targets(&self) -> Vec<String> {
        return self.targets.clone();
    }

    fn add_input(&mut self, _: String) {
        return;
    }

    fn process(&mut self, message: &Message) -> VecDeque<Message> {
        return self
            .targets
            .iter()
            .map(|t| Broadcaster::make_message(t.clone(), message.signal))
            .collect();
    }
}
