use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    pub from: String,
    pub left: String,
    pub right: String,
}

impl Edge {
    pub fn new(from: &str, left: &str, right: &str) -> Self {
        return Edge {
            from: from.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseEdgeError;

impl FromStr for Edge {
    type Err = ParseEdgeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spaces_and_labels_only = s
            .replace('=', " ")
            .replace('(', " ")
            .replace(')', " ")
            .replace(',', " ");

        let split = spaces_and_labels_only
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        let from = split[0];
        let left = split[1];
        let right = split[2];

        return Ok(Edge::new(from, left, right));
    }
}
