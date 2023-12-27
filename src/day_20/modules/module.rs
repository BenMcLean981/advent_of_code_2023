use std::collections::VecDeque;

use crate::day_20::message::Message;

pub trait Module {
    fn get_name(&self) -> String;

    fn get_targets(&self) -> Vec<String>;

    fn add_input(&mut self, input: String);

    fn process(&mut self, message: &Message) -> VecDeque<Message>;
}
