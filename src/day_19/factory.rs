use std::collections::HashMap;

use super::{part::Part, rule::Destination, workflow::Workflow};

pub struct Factory {
    workflows: HashMap<String, Workflow>,
}

impl Factory {
    pub fn new(workflows: Vec<Workflow>) -> Self {
        let mut map = HashMap::new();

        for w in workflows {
            map.insert(w.name.clone(), w);
        }

        return Factory { workflows: map };
    }

    pub fn process(&self, part: &Part) -> &Destination {
        return self.process_at(part, "in");
    }

    pub fn process_at(&self, part: &Part, workflow: &str) -> &Destination {
        let workflow = self.workflows.get(workflow).unwrap();
        let destination = workflow.process(part);

        match destination {
            Destination::Rejected => destination,
            Destination::Accepted => destination,
            Destination::Workflow(w) => &self.process_at(part, w),
        }
    }
}
