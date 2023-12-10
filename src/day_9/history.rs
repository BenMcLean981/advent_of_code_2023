use std::{iter::zip, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct History {
    values: Vec<i32>,
}

impl History {
    pub fn new(values: Vec<i32>) -> Self {
        return History { values };
    }

    pub fn compute_derivative(&self) -> History {
        let pairs = zip(self.values.iter(), self.values.iter().skip(1));
        let diffs = pairs.map(|p| p.1 - p.0).collect();

        return History::new(diffs);
    }

    pub fn is_zero(&self) -> bool {
        return self.values.iter().all(|v| *v == 0);
    }

    pub fn get_next(&self) -> i32 {
        if self.is_zero() {
            return 0;
        } else {
            self.get_next_from_derivative()
        }
    }

    fn get_next_from_derivative(&self) -> i32 {
        let derivative = self.compute_derivative();
        if derivative.is_zero() {
            return self.get_last();
        } else {
            return self.get_last() + derivative.get_next();
        }
    }

    fn get_last(&self) -> i32 {
        return *self.values.last().unwrap();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseHistoryError;

impl FromStr for History {
    type Err = ParseHistoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(i32::from_str)
            .map(|u| u.unwrap())
            .collect();

        return Ok(History::new(values));
    }
}
