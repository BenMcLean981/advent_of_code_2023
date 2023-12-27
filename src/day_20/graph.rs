use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::modules::{
    broadcaster::Broadcaster, conjunction::Conjunction, flip_flop::FlipFlop,
    module::Module,
};

pub struct Graph {
    modules: HashMap<String, Rc<RefCell<dyn Module>>>,
}

impl Graph {
    pub fn new() -> Self {
        return Graph {
            modules: HashMap::new(),
        };
    }

    pub fn add(&mut self, line: &str) {
        let module = make_module(line);

        let name = module.borrow().get_name();

        self.modules.insert(name, module);
    }

    pub fn connect(&mut self) {
        for m in self.modules.values() {
            let m = m.borrow();

            for t in m.get_targets() {
                if let Some(target) = self.get(t) {
                    target.borrow_mut().add_input(m.get_name());
                }
            }
        }
    }

    pub fn get(&self, name: String) -> Option<&Rc<RefCell<dyn Module>>> {
        return self.modules.get(&name);
    }
}

fn make_module(line: &str) -> Rc<RefCell<dyn Module>> {
    let split = line
        .split("->")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let targets = split[1]
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();

    let name = split[0];

    let first = name.chars().nth(0).unwrap();

    if first == '%' {
        return Rc::new(RefCell::new(FlipFlop::new(
            name[1..].to_string(),
            targets,
        )));
    } else if first == '&' {
        return Rc::new(RefCell::new(Conjunction::new(
            name[1..].to_string(),
            targets,
        )));
    } else if name == Broadcaster::get_name() {
        return Rc::new(RefCell::new(Broadcaster::new(targets)));
    } else {
        panic!();
    }
}
