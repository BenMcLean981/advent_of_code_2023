use std::str::FromStr;

use super::{
    part::Part,
    part_range::PartRange,
    part_range_map::PartRangeMap,
    rule::{Destination, Rule},
};

pub struct Workflow {
    pub name: String,
    rules: Vec<Rule>,
    default: Destination,
}

impl Workflow {
    pub fn process(&self, part: &Part) -> &Destination {
        for rule in &self.rules {
            if rule.applies(part) {
                return &rule.destination;
            }
        }

        return &self.default;
    }

    pub fn process_range(&self, range: PartRange) -> Vec<PartRangeMap> {
        let mut mapped: Vec<PartRangeMap> = vec![];
        let mut remainders = vec![range];

        for rule in &self.rules {
            let mut next_remainders = vec![];

            while !remainders.is_empty() {
                let r = remainders.pop().unwrap();

                let applied = rule.apply(&r);

                if let Some(applied) = applied.applied {
                    let next_mapped = PartRangeMap {
                        range: applied,
                        destination: rule.destination.clone(),
                    };

                    mapped.push(next_mapped);
                } else {
                    // this rule does not apply to r
                    next_remainders.push(r);
                }

                next_remainders.extend(applied.remainders);
            }

            remainders = next_remainders;
        }

        for range in remainders {
            let next_mapped = PartRangeMap {
                range,
                destination: self.default.clone(),
            };

            mapped.push(next_mapped)
        }

        return mapped;
    }
}

#[derive(Debug)]
pub struct ParseWorkflowError;

impl FromStr for Workflow {
    type Err = ParseWorkflowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s
            .split('{')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        let name = split[0].to_string();

        let rules = split[1].replace('}', "");
        let rules = rules
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        let default = Destination::from_str(rules.last().unwrap()).unwrap();
        let rules = rules
            .iter()
            .take(rules.len() - 1)
            .map(|r| Rule::from_str(r).unwrap())
            .collect::<Vec<Rule>>();

        return Ok(Workflow {
            name,
            rules,
            default,
        });
    }
}
