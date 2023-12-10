use super::{
    position::Position,
    tile_type::{Connection, TileType},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Grid {
    rows: Vec<Vec<TileType>>,
    num_cols: usize,
    num_rows: usize,
}

impl Grid {
    pub fn new(rows: Vec<Vec<TileType>>) -> Self {
        return Grid {
            num_rows: rows.len(),
            num_cols: rows[0].len(),
            rows,
        };
    }

    pub fn from_lines(lines: Vec<&str>) -> Self {
        let rows = lines
            .iter()
            .map(|l| Grid::read_line(l))
            .collect::<Vec<Vec<TileType>>>();

        return Grid::new(rows);
    }

    fn read_line(line: &str) -> Vec<TileType> {
        return line
            .chars()
            .map(|c| TileType::from(c))
            .collect::<Vec<TileType>>();
    }

    pub fn get_at_position(&self, position: &Position) -> TileType {
        if position.row.is_negative() || position.col.is_negative() {
            panic!();
        }

        return self.rows[position.row as usize][position.col as usize];
    }

    pub fn get_available_positions(&self, position: Position) -> Vec<Position> {
        let mut results: Vec<Position> = vec![];

        let tile_type = self.get_at_position(&position);
        let connections = tile_type.get_connections();

        let north = self.get_north(&position);
        if let Some(north) = north {
            if !self.is_ground(&north)
                && connections.contains(&Connection::North)
            {
                results.push(north);
            }
        }

        let east = self.get_east(&position);
        if let Some(east) = east {
            if !self.is_ground(&east) && connections.contains(&Connection::East)
            {
                results.push(east);
            }
        }

        let south = self.get_south(&position);
        if let Some(south) = south {
            if !self.is_ground(&south)
                && connections.contains(&Connection::South)
            {
                results.push(south);
            }
        }

        let west = self.get_west(&position);
        if let Some(west) = west {
            if !self.is_ground(&west) && connections.contains(&Connection::West)
            {
                results.push(west);
            }
        }

        return results;
    }

    fn get_north(&self, p: &Position) -> Option<Position> {
        if p.row > 0 {
            return Some(Position::new(p.row - 1, p.col));
        } else {
            return None;
        }
    }

    fn get_east(&self, p: &Position) -> Option<Position> {
        if p.col < self.num_cols as i32 - 1 {
            return Some(Position::new(p.row, p.col + 1));
        } else {
            return None;
        }
    }

    fn get_south(&self, p: &Position) -> Option<Position> {
        if p.row < self.num_rows as i32 - 1 {
            return Some(Position::new(p.row + 1, p.col));
        } else {
            return None;
        }
    }

    fn get_west(&self, p: &Position) -> Option<Position> {
        if p.col > 0 {
            return Some(Position::new(p.row, p.col - 1));
        } else {
            return None;
        }
    }

    fn is_ground(&self, p: &Position) -> bool {
        return self.get_at_position(p) == TileType::Ground;
    }
}
