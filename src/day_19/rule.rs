use std::str::FromStr;

use super::{part::Part, part_range::PartRange, range::Range};

pub struct Rule {
    quality: Quality,
    operator: Operator,
    quantity: u32,
    pub destination: Destination,
}

impl Rule {
    pub fn applies(&self, part: &Part) -> bool {
        match self.operator {
            Operator::LessThan => self.get_quality(part) < self.quantity,
            Operator::GreaterThan => self.get_quality(part) > self.quantity,
        }
    }

    fn get_quality(&self, part: &Part) -> u32 {
        match self.quality {
            Quality::X => part.x,
            Quality::M => part.m,
            Quality::A => part.a,
            Quality::S => part.s,
        }
    }

    pub fn apply(&self, part_range: &PartRange) -> RuleApplication {
        let range = self.get_range_quality(part_range);
        let intersection = Range::intersection(range, self.get_range());

        if let Some(intersection) = intersection {
            let applied = self.set_range(part_range, intersection);

            let difference = Range::difference(range, intersection);
            let remainders = difference
                .iter()
                .map(|d| self.set_range(part_range, *d))
                .collect();

            return RuleApplication {
                applied: Some(applied),
                remainders,
            };
        } else {
            return RuleApplication {
                applied: None,
                remainders: vec![*part_range],
            };
        }
    }

    pub fn set_range(&self, part_range: &PartRange, range: Range) -> PartRange {
        match self.quality {
            Quality::X => PartRange {
                x: range,
                m: part_range.m,
                a: part_range.a,
                s: part_range.s,
            },
            Quality::M => PartRange {
                x: part_range.x,
                m: range,
                a: part_range.a,
                s: part_range.s,
            },
            Quality::A => PartRange {
                x: part_range.x,
                m: part_range.m,
                a: range,
                s: part_range.s,
            },
            Quality::S => PartRange {
                x: part_range.x,
                m: part_range.m,
                a: part_range.a,
                s: range,
            },
        }
    }

    fn get_range(&self) -> Range {
        match self.operator {
            Operator::LessThan => Range::new(1, self.quantity),
            Operator::GreaterThan => Range::new(self.quantity, 4000),
        }
    }

    fn get_range_quality(&self, range: &PartRange) -> Range {
        match self.quality {
            Quality::X => range.x,
            Quality::M => range.m,
            Quality::A => range.a,
            Quality::S => range.s,
        }
    }
}

pub struct RuleApplication {
    pub applied: Option<PartRange>,
    pub remainders: Vec<PartRange>,
}

#[derive(Debug)]
pub struct ParseRuleError;

impl FromStr for Rule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let quality = Quality::from(s.chars().nth(0).unwrap());
        let operator = Operator::from(s.chars().nth(1).unwrap());

        let split = s.split(':').collect::<Vec<&str>>();
        let quantity = u32::from_str(&split[0][2..]).unwrap();
        let destination = Destination::from_str(split[1]).unwrap();

        return Ok(Rule {
            quality,
            operator,
            quantity,
            destination,
        });
    }
}

enum Quality {
    X,
    M,
    A,
    S,
}

impl From<char> for Quality {
    fn from(value: char) -> Self {
        match value {
            'x' => Quality::X,
            'm' => Quality::M,
            'a' => Quality::A,
            's' => Quality::S,
            _ => panic!(),
        }
    }
}

pub enum Operator {
    LessThan,
    GreaterThan,
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            '<' => Operator::LessThan,
            '>' => Operator::GreaterThan,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Destination {
    Rejected,
    Accepted,
    Workflow(String),
}

impl Destination {
    pub fn is_continuing(&self) -> bool {
        return self == &Destination::Accepted
            || self == &Destination::Rejected;
    }

    pub fn is_accepted(&self) -> bool {
        return self == &Destination::Accepted;
    }
}

#[derive(Debug)]
pub struct ParseDestinationError;

impl FromStr for Destination {
    type Err = ParseDestinationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "A" {
            return Ok(Destination::Accepted);
        } else if s == "R" {
            return Ok(Destination::Rejected);
        } else {
            return Ok(Destination::Workflow(s.to_string()));
        }
    }
}
