use super::{grid::Grid, position::Position};

pub struct LoopFinder {
    grid: Grid,
    path: Vec<Position>,
}

impl LoopFinder {
    fn new(grid: Grid) -> Self {
        let start = grid.get_start_position();

        return LoopFinder {
            grid,
            path: vec![start],
        };
    }

    pub fn find_loop(grid: &Grid) -> Vec<Position> {
        let mut finder = LoopFinder::new(grid.clone());

        while !finder.is_done() || !finder.has_started() {
            let next = finder.find_next_position();
            finder.path.push(next);
        }

        return finder.path;
    }

    fn is_done(&self) -> bool {
        return self.get_start() == self.get_current();
    }

    fn get_start(&self) -> Position {
        return *self.path.first().unwrap();
    }

    fn get_current(&self) -> Position {
        return *self.path.last().unwrap();
    }

    fn get_previous(&self) -> Option<Position> {
        if self.path.len() < 2 {
            return None;
        } else {
            return Some(self.path[self.path.len() - 2]);
        }
    }

    fn has_started(&self) -> bool {
        return self.path.len() > 1;
    }

    fn find_next_position(&self) -> Position {
        return *self
            .grid
            .get_available_positions(self.get_current())
            .iter()
            .find(|p| {
                **p != self.get_current() && Some(**p) != self.get_previous()
            })
            .unwrap();
    }
}
