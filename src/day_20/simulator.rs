use std::collections::VecDeque;

use crate::utils::num::multiple_lcm;

use super::{
    graph::Graph, message::Message, modules::broadcaster::Broadcaster,
    signal::Signal,
};

pub struct Simulator {
    backup: Graph,
    graph: Graph,
    queue: VecDeque<Message>,
}

impl Simulator {
    pub fn new(graph: Graph) -> Self {
        return Simulator {
            backup: graph.clone(),
            graph,
            queue: VecDeque::new(),
        };
    }

    pub fn simulate_button(&mut self) -> VecDeque<Message> {
        let first = Message::new(
            "button".to_string(),
            Broadcaster::get_name(),
            Signal::Low,
        );

        self.queue.push_back(first);

        return self.process();
    }

    fn process(&mut self) -> VecDeque<Message> {
        let mut result = VecDeque::new();

        while let Some(message) = self.queue.pop_front() {
            if let Some(target) = self.graph.get(message.target.clone()) {
                let next_messages = target.borrow_mut().process(&message);
                self.queue.extend(next_messages);
            }

            result.push_back(message);
        }

        return result;
    }

    pub fn simulate_until_rx(&mut self) -> u64 {
        let sources = self.find_sources();

        let cyclics = sources
            .iter()
            .map(|name| self.find_cyclic_of(name, Signal::Low))
            .collect::<Vec<Cyclic>>();

        let periods = cyclics.iter().map(|c| c.period);

        return multiple_lcm(periods.collect());
    }

    fn find_sources(&self) -> Vec<String> {
        let aggregator = self.find_aggregator();

        return self.graph.find_to(aggregator);
    }

    fn find_aggregator(&self) -> String {
        let aggregator = self.graph.find_to("rx".to_string());

        if aggregator.len() != 1 {
            panic!()
        }

        return aggregator[0].clone();
    }

    fn find_cyclic_of(&mut self, name: &String, signal: Signal) -> Cyclic {
        self.reset();

        let mut offset = 0;
        let mut period = 0;
        let mut found_offset = false;

        loop {
            let processed = self.simulate_button();

            if !found_offset {
                offset += 1;
            } else {
                period += 1;
            }

            if processed
                .iter()
                .any(|p| p.target == name.to_string() && p.signal == signal)
            {
                if found_offset {
                    return Cyclic { offset, period };
                } else {
                    // switch modes
                    found_offset = true;
                }
            }
        }
    }

    fn reset(&mut self) {
        self.graph = self.backup.clone();
    }
}

struct Cyclic {
    period: u64,
    offset: u64,
}
