use std::collections::HashMap;

use super::{
    part::Part, part_range::PartRange, part_range_map::PartRangeMap,
    rule::Destination, workflow::Workflow,
};

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

    fn process_at(&self, part: &Part, workflow: &str) -> &Destination {
        let workflow = self.workflows.get(workflow).unwrap();
        let destination = workflow.process(part);

        match destination {
            Destination::Rejected => destination,
            Destination::Accepted => destination,
            Destination::Workflow(w) => &self.process_at(part, w),
        }
    }

    pub fn process_range(&self, range: &PartRange) -> Vec<PartRange> {
        let maps = vec![PartRangeMap::new(
            *range,
            Destination::Workflow("in".to_string()),
        )];

        let processed = self.process_ranges_at(&maps);

        return processed.iter().map(|p| p.range.clone()).collect();
    }

    fn process_ranges_at(&self, maps: &Vec<PartRangeMap>) -> Vec<PartRangeMap> {
        let continuing = maps
            .iter()
            .filter(|m| m.destination.is_continuing())
            .collect::<Vec<&PartRangeMap>>();
        let mut accepted = maps
            .iter()
            .filter(|m| m.destination.is_accepted())
            .cloned()
            .collect::<Vec<PartRangeMap>>();

        if continuing.len() == 0 {
            return accepted;
        }

        let processed = continuing
            .iter()
            .flat_map(|map| self.process_range_at(map))
            .collect::<Vec<PartRangeMap>>();

        accepted.extend(self.process_ranges_at(&processed));

        return accepted;
    }

    fn process_range_at(&self, map: &PartRangeMap) -> Vec<PartRangeMap> {
        let workflow = self.get_workflow(&map.destination);

        return workflow.process_range(map.range);
    }

    fn get_workflow(&self, destination: &Destination) -> &Workflow {
        match destination {
            Destination::Rejected => panic!(),
            Destination::Accepted => panic!(),
            Destination::Workflow(w) => self.workflows.get(w).unwrap(),
        }
    }
}
