use std::collections::HashMap;

use super::{direction::Direction, edge::Edge};

pub struct Map {
    left_map: HashMap<String, String>,
    right_map: HashMap<String, String>,
}

impl Map {
    pub fn from_edges(edges: &Vec<Edge>) -> Self {
        return Map {
            left_map: Self::make_left_map(edges),
            right_map: Self::make_right_map(edges),
        };
    }

    fn make_left_map(edges: &Vec<Edge>) -> HashMap<String, String> {
        let mut result = HashMap::<String, String>::new();

        for edge in edges {
            result.insert(edge.from.to_string(), edge.left.to_string());
        }

        return result;
    }

    fn make_right_map(edges: &Vec<Edge>) -> HashMap<String, String> {
        let mut result = HashMap::<String, String>::new();

        for edge in edges {
            result.insert(edge.from.to_string(), edge.right.to_string());
        }

        return result;
    }

    pub fn get_next(&self, node: &str, direction: Direction) -> &str {
        return match direction {
            Direction::Left => self.get_left(node),
            Direction::Right => self.get_right(node),
        };
    }

    fn get_left(&self, node: &str) -> &str {
        return self.left_map.get(node).unwrap();
    }

    fn get_right(&self, node: &str) -> &str {
        return self.right_map.get(node).unwrap();
    }
}
