use std::collections::{HashMap, HashSet};

use crate::day_17::heat_map::{Edge, HeatMap};

use super::{path::Path, position::Position};

pub struct DijsktraSolver {
    map: HeatMap,
    losses: HashMap<Position, u32>,
    prevs: HashMap<Position, Option<Position>>,
    vertices: HashSet<Position>,
}

impl DijsktraSolver {
    pub fn new(map: HeatMap) -> DijsktraSolver {
        let source = Position::new(0, 0);

        let mut prevs = HashMap::<Position, Option<Position>>::new();

        let mut losses = HashMap::<Position, u32>::new();

        for pos in map.get_positions() {
            losses.insert(pos, u32::MAX);
            prevs.insert(pos, None);
        }

        losses.insert(source, map.get_heat_loss(source));

        let vertices = map.get_positions().iter().copied().collect();

        return DijsktraSolver {
            map,
            losses,
            prevs,
            vertices,
        };
    }

    pub fn solve(&mut self, destination: Position) -> Path {
        while !self.vertices.is_empty() {
            let u = self.find_min();
            self.vertices.remove(&u);

            if u == destination {
                let path = self.make_path(destination);
                self.map.debug_path(path.clone());

                return path;
            }

            let path = &self.make_path(u);

            let edges = self.map.get_edges(path);

            for edge in edges {
                let added = path.move_by(edge.direction);
                let v = added.get_current();

                let u_loss = self.losses.get(&u).unwrap();
                let new_loss = u_loss + edge.loss;

                if new_loss < *self.losses.get(&v).unwrap() {
                    self.losses.insert(v, new_loss);
                    self.prevs.insert(v, Some(u));
                }
            }
        }

        panic!("No path found.");
    }

    fn make_path(&self, position: Position) -> Path {
        let mut positions = vec![position];

        let mut prev = self.prevs.get(positions.last().unwrap()).unwrap();

        while let Some(next) = prev {
            positions.insert(0, *next);

            let current = positions.first().unwrap();
            prev = self.prevs.get(current).unwrap();
        }

        return Path::from_positions(positions);
    }

    fn find_min(&self) -> Position {
        let mut pos = *self.vertices.iter().nth(0).unwrap();
        let mut min = *self.losses.get(&pos).unwrap();

        for v in self.vertices.iter() {
            let loss = *self.losses.get(v).unwrap();

            if loss < min {
                min = loss;
                pos = *v;
            }
        }

        return pos;
    }
}
