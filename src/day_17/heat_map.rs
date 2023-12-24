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
                positions.push(Position::new(i as i32, j as i32));
            }
        }

        return positions;
    }

    pub fn is_position_valid(&self, pos: Position) -> bool {
        return self.is_row_valid(pos.row) && self.is_col_valid(pos.col);
    }

    fn is_row_valid(&self, row: i32) -> bool {
        return 0 <= row && row <= self.get_num_rows() as i32 - 1;
    }

    fn is_col_valid(&self, col: i32) -> bool {
        return 0 <= col && col <= self.get_num_cols() as i32 - 1;
    }

    fn get_num_rows(&self) -> usize {
        return self.rows.len();
    }

    fn get_num_cols(&self) -> usize {
        return self.rows[0].len();
    }

    pub fn get_total_loss(&self, path: Path) -> u32 {
        let losses = path
            .positions
            .iter()
            .map(|p| self.get_heat_loss(*p))
            .collect::<Vec<u32>>();

        return losses.iter().sum();
    }

    pub fn get_heat_loss(&self, position: Position) -> u32 {
        return self.rows[position.row as usize][position.col as usize];
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
            && path.get_current().col != self.get_num_cols() as i32 - 1
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
            && path.get_current().row != self.get_num_rows() as i32 - 1
        {
            return Some(Edge::new(
                Direction::Down,
                self.get_heat_loss(path.get_current().move_by(Direction::Down)),
            ));
        }

        return None;
    }

    pub fn get_bot_right(&self) -> Position {
        return Position::new(
            self.get_num_rows() as i32 - 1,
            self.get_num_cols() as i32 - 1,
        );
    }

    pub fn debug_path(&self, path: Path) {
        for (i, row) in self.rows.iter().enumerate() {
            let mut chars: Vec<char> = vec![];

            for (j, _) in row.iter().enumerate() {
                let pos = Position::new(i as i32, j as i32);

                let d = path.get_direction_before(pos);

                if let Some(d) = d {
                    chars.push(to_char(d))
                } else {
                    chars.push(
                        char::from_digit(self.get_heat_loss(pos), 10).unwrap(),
                    )
                }
            }

            let line = chars.iter().collect::<String>();
            println!("{line}");
        }
    }
}

fn to_char(d: Direction) -> char {
    match d {
        Direction::Right => '>',
        Direction::Down => 'v',
        Direction::Up => '^',
        Direction::Left => '<',
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Edge {
    pub direction: Direction,
    pub loss: u32,
}

impl Edge {
    pub fn new(direction: Direction, loss: u32) -> Self {
        return Edge { direction, loss };
    }
}
