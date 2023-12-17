use std::collections::HashSet;

pub struct MirrorGrid {
    rows: Vec<Vec<OpticalOperator>>,
    beams: Vec<Beam>,
    simulated: HashSet<Beam>,
}

impl MirrorGrid {
    pub fn new(rows: Vec<Vec<OpticalOperator>>) -> Self {
        return MirrorGrid {
            rows,
            beams: vec![Beam::new(Direction::Right, Position::new(0, 0))],
            simulated: HashSet::<Beam>::new(),
        };
    }

    pub fn from_lines(lines: Vec<&str>) -> Self {
        let rows = lines.iter().map(|l| read_line(l)).collect();

        fn read_line(line: &str) -> Vec<OpticalOperator> {
            return line.chars().map(OpticalOperator::from).collect();
        }

        return MirrorGrid::new(rows);
    }

    pub fn simulate(&mut self) {
        while self.beams.len() > 0 {
            let beam = self.beams[0].clone();
            self.beams.remove(0); // always remove, we are going to make new beam(s)

            if !self.is_out_of_bounds(&beam) && !self.simulated.contains(&beam)
            {
                let new_beams = self.simulate_beam(&beam);

                // cache for later.
                self.simulated.insert(beam);

                self.beams.extend(new_beams);
            }
        }
    }

    fn is_out_of_bounds(&self, beam: &Beam) -> bool {
        return beam.position.row < 0
            || beam.position.row >= self.rows.len() as i32
            || beam.position.col < 0
            || beam.position.col >= self.rows[0].len() as i32;
    }

    fn simulate_beam(&self, beam: &Beam) -> Vec<Beam> {
        if self.is_out_of_bounds(beam) {
            return vec![]; // done with this beam
        }

        let operator = self.get_operator(beam.position);

        if operator.should_split(beam) {
            let (beam1, beam2) = operator.split(beam);

            return vec![beam1.translate(), beam2.translate()];
        } else if operator.is_mirror() {
            let rotated = operator.rotate(beam);

            return vec![rotated.translate()];
        } else {
            return vec![beam.translate()];
        }
    }

    fn get_operator(&self, position: Position) -> OpticalOperator {
        return self.rows[position.row as usize][position.col as usize];
    }

    pub fn count_energized(&self) -> usize {
        return self
            .simulated
            .iter()
            .map(|b| b.position)
            .collect::<HashSet<Position>>()
            .len();
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum OpticalOperator {
    None,
    HorizontalSplitter,
    VerticalSplitter,
    DownwardsMirror,
    UpwardsMirror,
}

impl OpticalOperator {
    fn should_split(&self, beam: &Beam) -> bool {
        return beam.direction.is_horizontal()
            && *self == OpticalOperator::VerticalSplitter
            || beam.direction.is_vertical()
                && *self == OpticalOperator::HorizontalSplitter;
    }

    fn split(&self, beam: &Beam) -> (Beam, Beam) {
        match self {
            OpticalOperator::HorizontalSplitter => (
                Beam::new(Direction::Left, beam.position),
                Beam::new(Direction::Right, beam.position),
            ),
            OpticalOperator::VerticalSplitter => (
                Beam::new(Direction::Up, beam.position),
                Beam::new(Direction::Down, beam.position),
            ),
            _ => panic!(),
        }
    }

    fn is_mirror(&self) -> bool {
        return *self == OpticalOperator::DownwardsMirror
            || *self == OpticalOperator::UpwardsMirror;
    }

    fn rotate(&self, beam: &Beam) -> Beam {
        match *self {
            OpticalOperator::DownwardsMirror => {
                Beam::new(beam.direction.rotate_downwards(), beam.position)
            }
            OpticalOperator::UpwardsMirror => {
                Beam::new(beam.direction.rotate_upwards(), beam.position)
            }
            _ => panic!(),
        }
    }
}

impl From<char> for OpticalOperator {
    fn from(value: char) -> Self {
        return match value {
            '.' => OpticalOperator::None,
            '|' => OpticalOperator::VerticalSplitter,
            '-' => OpticalOperator::HorizontalSplitter,
            '\\' => OpticalOperator::DownwardsMirror,
            '/' => OpticalOperator::UpwardsMirror,
            _ => panic!(),
        };
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Beam {
    direction: Direction,
    position: Position,
}

impl Beam {
    pub fn new(direction: Direction, position: Position) -> Self {
        return Beam {
            direction,
            position,
        };
    }

    pub fn translate(&self) -> Self {
        return Beam::new(
            self.direction,
            self.position.translate(self.direction),
        );
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_downwards(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        }
    }

    pub fn rotate_upwards(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        return *self == Direction::Left || *self == Direction::Right;
    }

    pub fn is_vertical(&self) -> bool {
        return *self == Direction::Up || *self == Direction::Down;
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    pub fn new(row: i32, col: i32) -> Self {
        return Position { row, col };
    }

    pub fn translate(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Position::new(self.row - 1, self.col),
            Direction::Down => Position::new(self.row + 1, self.col),
            Direction::Left => Position::new(self.row, self.col - 1),
            Direction::Right => Position::new(self.row, self.col + 1),
        }
    }
}
