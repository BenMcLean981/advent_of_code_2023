use std::collections::VecDeque;

use super::{
    graph::Graph, message::Message, modules::broadcaster::Broadcaster,
    signal::Signal,
};

pub struct Simulator {
    graph: Graph,
    queue: VecDeque<Message>,
    pub processed: VecDeque<Message>,
}

impl Simulator {
    pub fn new(graph: Graph) -> Self {
        return Simulator {
            graph,
            queue: VecDeque::new(),
            processed: VecDeque::new(),
        };
    }

    pub fn simulate(&mut self) {
        let first = Message::new(
            "button".to_string(),
            Broadcaster::get_name(),
            Signal::Low,
        );

        self.queue.push_back(first);

        self.process();
    }

    fn process(&mut self) {
        while let Some(message) = self.queue.pop_front() {
            if let Some(target) = self.graph.get(message.target.clone()) {
                let next_messages = target.borrow_mut().process(&message);
                self.queue.extend(next_messages);
            }

            self.processed.push_back(message);
        }
    }
}
