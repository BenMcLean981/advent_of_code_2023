use std::collections::HashSet;

use super::{direction::Direction, position::Position};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Path {
    pub positions: Vec<Position>,
    directions: Vec<Direction>,
}

impl Path {
    pub fn new() -> Path {
        return Path {
            positions: vec![Position::new(0, 0)],
            directions: vec![],
        };
    }

    pub fn from_positions(positions: Vec<Position>) -> Self {
        // first one should be (0, 0)

        return positions.iter().skip(1).fold(Path::new(), |path, p| {
            path.move_by(Direction::make_delta(path.get_current(), *p))
        });
    }

    pub fn move_by(&self, d: Direction) -> Path {
        let mut result = self.clone();

        result.directions.push(d);
        result.positions.push(self.get_current().move_by(d));

        return result;
    }

    pub fn get_current(&self) -> Position {
        return *self.positions.last().unwrap();
    }

    pub fn can_move(&self, d: Direction) -> bool {
        let last = self.directions.last();

        if let Some(last) = last {
            if last.get_opposite() == d {
                return false;
            }
        }

        if self.directions.len() < 3 {
            return true;
        }

        let last_3 = self
            .directions
            .iter()
            .rev()
            .take(3)
            .copied()
            .collect::<HashSet<Direction>>();

        if last_3.len() != 1 {
            return true;
        } else {
            return !last_3.contains(&d);
        }
    }

    pub fn get_direction_before(&self, p: Position) -> Option<Direction> {
        for (i, pos) in self.positions.iter().enumerate() {
            if *pos == p && i != 0 {
                return Some(self.directions[i - 1]);
            }
        }

        return None;
    }

    pub fn is_done(&self) -> bool {
        let last = self.directions.last();

        if self.directions.len() == 3 {
            return true;
        }

        let last_3 = self
            .directions
            .iter()
            .rev()
            .take(3)
            .copied()
            .collect::<HashSet<Direction>>();

        return last_3.len() == 1;
    }
}
