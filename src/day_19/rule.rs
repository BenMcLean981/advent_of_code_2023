use std::str::FromStr;

use super::part::Part;

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
