use std::iter::zip;

use super::{grid::Grid, position::Position, tile_type::TileType};

pub struct AreaCalculator {
    grid: Grid,
    path: Vec<Position>,
}

impl AreaCalculator {
    pub fn new(grid: &Grid, path: Vec<Position>) -> AreaCalculator {
        return AreaCalculator {
            grid: grid.clone(),
            path,
        };
    }

    pub fn get_area(&self) -> u32 {
        return self
            .get_positions()
            .iter()
            .filter(|p| self.is_inside(p))
            .count() as u32;
    }

    fn get_positions(&self) -> Vec<Position> {
        let mut results: Vec<Position> = vec![];

        for row in 0..self.grid.num_rows {
            for col in 0..self.grid.num_cols {
                let position = Position::new(row as i32, col as i32);

                results.push(position);
            }
        }

        return results;
    }

    fn is_inside(&self, p: &Position) -> bool {
        if self.path.contains(p) || self.is_on_border(p) {
            return false;
        }

        let west_ray = self.make_west_ray(p);
        let crossings = self.count_crossings(&west_ray);

        return is_odd(crossings);
    }

    fn is_on_border(&self, p: &Position) -> bool {
        return p.row == 0
            || p.col == 0
            || p.row == self.grid.num_rows as i32 - 1
            || p.col == self.grid.num_cols as i32 - 1;
    }

    fn make_west_ray(&self, p: &Position) -> Vec<Position> {
        let mut results = vec![];

        for col in 0..p.col {
            let position = Position::new(p.row, col);
            results.push(position);
        }

        return results;
    }

    fn count_crossings(&self, ray: &Vec<Position>) -> u32 {
        return self.count_vertical_crossings(ray)
            + self.count_corner_crossings(ray);
    }

    fn count_vertical_crossings(&self, ray: &Vec<Position>) -> u32 {
        let mut crossings = 0;

        for position in ray {
            if self.grid.get_at_position(&position) == TileType::Vertical
                && self.path.contains(&position)
            {
                crossings += 1;
            }
        }

        return crossings;
    }

    fn count_corner_crossings(&self, ray: &Vec<Position>) -> u32 {
        let mut crossings = 0;

        let corners = ray
            .iter()
            .filter(|p| self.is_corner(p))
            .map(|p| *p)
            .collect::<Vec<Position>>();

        for stretch in zip(corners.iter(), corners.iter().skip(1)) {
            let start = stretch.0;
            let end = stretch.1;

            if self.is_stretch_crossing(start, end) {
                crossings += 1;
            }
        }

        return crossings;
    }

    fn is_stretch_crossing(&self, start: &Position, end: &Position) -> bool {
        if !self.path.contains(start) || !self.path.contains(end) {
            return false;
        }

        let is_south_north = self.grid.get_at_position(start)
            == TileType::SouthEast
            && self.grid.get_at_position(end) == TileType::NorthWest;

        let is_north_south = self.grid.get_at_position(start)
            == TileType::NorthEast
            && self.grid.get_at_position(end) == TileType::SouthWest;

        return is_south_north || is_north_south;
    }

    fn is_corner(&self, position: &Position) -> bool {
        let t = self.grid.get_at_position(position);

        return t == TileType::NorthEast
            || t == TileType::NorthWest
            || t == TileType::SouthEast
            || t == TileType::SouthWest;
    }
}

fn is_odd(n: u32) -> bool {
    return n % 2 != 0;
}
