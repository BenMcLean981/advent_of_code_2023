use std::collections::HashMap;

use crate::day_17::heat_map::HeatMap;

use super::{heat_map::Edge, path::Path, position::Position};

pub struct DijsktraSolver {
    map: HeatMap,
    losses: HashMap<Position, u32>,
    paths: HashMap<Position, Path>,
    vertices: Vec<Position>,
}

impl DijsktraSolver {
    pub fn new(map: HeatMap) -> DijsktraSolver {
        let source = Position::new(0, 0);

        let mut paths = HashMap::<Position, Path>::new();

        let mut losses = HashMap::<Position, u32>::new();

        for pos in map.get_positions() {
            losses.insert(pos, u32::MAX);
        }

        paths.insert(source, Path::from_positions(vec![source]));
        losses.insert(source, 0);

        let vertices = map.get_positions().iter().copied().collect();

        return DijsktraSolver {
            map,
            losses,
            paths,
            vertices,
        };
    }

    pub fn solve(&mut self, destination: Position) -> Path {
        while !self.vertices.is_empty() {
            let u = self.pop_min();

            if u == destination {
                return self.get_path(destination).clone();
            }

            self.add_adjacents(u);
        }

        panic!("No path found.");
    }

    fn add_adjacents(&mut self, u: Position) {
        let path = self.get_path(u);
        let edges = self.map.get_edges(&path);

        for edge in edges {
            self.process_edge(edge, u);
        }
    }

    fn process_edge(&mut self, edge: Edge, from: Position) {
        let path = self.get_path(from);

        let new_path = path.move_by(edge.direction);
        let to = new_path.get_current();

        let u_loss = self.losses.get(&from).unwrap();
        let new_loss = u_loss + edge.loss;

        let old_loss = *self.losses.get(&to).unwrap();

        if new_loss <= old_loss {
            self.losses.insert(to, new_loss);
            self.paths.insert(to, new_path);
        }
    }

    fn pop_min(&mut self) -> Position {
        self.order_vertices();

        let result = *self.vertices.first().unwrap();

        if self.paths.get(&result).is_none() {
            panic!();
        }

        self.vertices.remove(0);
        return result;
    }

    fn order_vertices(&mut self) {
        let mut vertices = self.vertices.clone();

        vertices.sort_by(|p1, p2| self.get_loss(p1).cmp(&self.get_loss(p2)));

        self.vertices = vertices;
    }

    fn get_path(&self, position: Position) -> &Path {
        return self.paths.get(&position).unwrap();
    }

    fn get_loss(&self, p: &Position) -> u32 {
        return *self.losses.get(p).unwrap();
    }
}

struct State {
    position: Position,
    loss: usize,
    path: Option<Path>,
}
