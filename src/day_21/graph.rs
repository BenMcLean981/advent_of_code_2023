use std::collections::HashSet;

use super::position::Position;

pub struct Graph {
    start: Position,
    rocks: HashSet<Position>,
    rows: usize,
    cols: usize,
}

impl Graph {
    pub fn from_lines(lines: &Vec<&str>) -> Self {
        let mut cells: Vec<Cell> = vec![];

        for (i, row) in lines.iter().enumerate() {
            for (j, c) in row.chars().enumerate() {
                let col = j as i32;
                let row = i as i32;

                cells.push(Cell {
                    position: Position::new(row, col),
                    is_rock: c == '#',
                    is_start: c == 'S',
                });
            }
        }

        let rows = lines.iter().count();
        let cols = lines.first().unwrap().len();

        let start = cells.iter().find(|c| c.is_start).unwrap().position;
        let rocks = cells
            .iter()
            .filter(|c| c.is_rock)
            .map(|c| c.position.clone())
            .collect();

        return Graph {
            rows,
            cols,
            start,
            rocks,
        };
    }

    pub fn step(&self, times: u32) -> HashSet<Position> {
        let mut current = HashSet::<Position>::new();
        current.insert(self.start);

        for _ in 0..times {
            let mut next = HashSet::new();

            for p in current {
                let neighbors = p.get_neighbors();
                let possible = neighbors.iter().filter(|n| !self.is_rock(n));
                next.extend(possible);
            }

            current = next;
        }

        return current;
    }

    pub fn step_infinitely(&self, times: u32) -> HashSet<Position> {
        let mut current = HashSet::<Position>::new();
        current.insert(self.start);

        for _ in 0..times {
            let mut next = HashSet::new();

            for p in current {
                let neighbors = p.get_neighbors();
                let possible =
                    neighbors.iter().filter(|n| !self.is_rock_infinitely(n));
                next.extend(possible);
            }

            current = next;
        }

        return current;
    }

    fn is_rock_infinitely(&self, p: &Position) -> bool {
        let modded =
            Position::new(p.row % self.rows as i32, p.col % self.cols as i32);

        return self.is_rock(&modded);
    }

    fn is_rock(&self, p: &Position) -> bool {
        return self.rocks.contains(p);
    }
}

struct Cell {
    position: Position,
    is_rock: bool,
    is_start: bool,
}
