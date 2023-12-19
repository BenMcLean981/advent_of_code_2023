use super::{direction::Direction, path::Path, position::Position};

#[derive(Clone)]
pub struct HeatMap {
    rows: Vec<Vec<u32>>,
}

impl HeatMap {
    pub fn new(rows: Vec<Vec<u32>>) -> Self {
        return HeatMap { rows };
    }

    pub fn from_lines(lines: Vec<&str>) -> Self {
        let rows = lines.iter().map(|l| read_line(l)).collect();

        fn read_line(l: &str) -> Vec<u32> {
            return l.chars().map(|c| c.to_digit(10).unwrap()).collect();
        }

        return HeatMap { rows };
    }

    pub fn get_positions(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = vec![];

        for (i, row) in self.rows.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                positions.push(Position::new(i, j));
            }
        }

        return positions;
    }

    pub fn get_total_loss(&self, path: Path) -> u32 {
        return path.positions.iter().map(|p| self.get_heat_loss(*p)).sum();
    }

    pub fn get_heat_loss(&self, position: Position) -> u32 {
        return self.rows[position.row][position.col];
    }

    pub fn get_edges(&self, path: &Path) -> Vec<Edge> {
        return vec![
            self.get_left_edge(path),
            self.get_right_edge(path),
            self.get_down_edge(path),
            self.get_up_edge(path),
        ]
        .iter()
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect();
    }

    pub fn get_left_edge(&self, path: &Path) -> Option<Edge> {
        if path.can_move(Direction::Left) && path.get_current().col != 0 {
            return Some(Edge::new(
                Direction::Left,
                self.get_heat_loss(path.get_current().move_by(Direction::Left)),
            ));
        }

        return None;
    }

    pub fn get_right_edge(&self, path: &Path) -> Option<Edge> {
        if path.can_move(Direction::Right)
            && path.get_current().col != self.rows[0].len() - 1
        {
            return Some(Edge::new(
                Direction::Right,
                self.get_heat_loss(
                    path.get_current().move_by(Direction::Right),
                ),
            ));
        }

        return None;
    }

    pub fn get_up_edge(&self, path: &Path) -> Option<Edge> {
        if path.can_move(Direction::Up) && path.get_current().row != 0 {
            return Some(Edge::new(
                Direction::Up,
                self.get_heat_loss(path.get_current().move_by(Direction::Up)),
            ));
        }

        return None;
    }

    pub fn get_down_edge(&self, path: &Path) -> Option<Edge> {
        if path.can_move(Direction::Down)
            && path.get_current().row != self.rows.len() - 1
        {
            return Some(Edge::new(
                Direction::Down,
                self.get_heat_loss(path.get_current().move_by(Direction::Down)),
            ));
        }

        return None;
    }

    pub fn get_bot_right(&self) -> Position {
        return Position::new(self.rows.len() - 1, self.rows[0].len() - 1);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Edge {
    pub direction: Direction,
    pub loss: u32,
}

impl Edge {
    pub fn new(direction: Direction, loss: u32) -> Self {
        return Edge { direction, loss };
    }
}
