use std::{cmp, collections::HashSet};

use super::{
    beam::Beam, direction::Direction, optical_operator::OpticalOperator,
    position::Position,
};

#[derive(Clone)]
pub struct MirrorGrid {
    rows: Vec<Vec<OpticalOperator>>,
    beams: Vec<Beam>,
    simulated: HashSet<Beam>,
}

impl MirrorGrid {
    pub fn new(rows: Vec<Vec<OpticalOperator>>) -> Self {
        return MirrorGrid {
            rows,
            beams: vec![],
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

    pub fn count_best_energized(&self) -> usize {
        let mut best = 0;

        for row in 0..self.rows.len() {
            let right = self.count_energized_for_beam(self.make_right(row));
            let left = self.count_energized_for_beam(self.make_left(row));

            best = cmp::max(best, right);
            best = cmp::max(best, left);
        }

        for col in 0..self.rows[0].len() {
            let up = self.count_energized_for_beam(self.make_up(col));
            let down = self.count_energized_for_beam(self.make_down(col));

            best = cmp::max(best, down);
            best = cmp::max(best, up);
        }

        return best;
    }

    fn make_right(&self, row: usize) -> Beam {
        return Beam::new(Direction::Right, Position::new(row as i32, 0));
    }

    fn make_left(&self, row: usize) -> Beam {
        return Beam::new(
            Direction::Left,
            Position::new(row as i32, self.rows[0].len() as i32 - 1),
        );
    }

    fn make_down(&self, col: usize) -> Beam {
        return Beam::new(Direction::Down, Position::new(0 as i32, col as i32));
    }

    fn make_up(&self, col: usize) -> Beam {
        return Beam::new(
            Direction::Up,
            Position::new(self.rows.len() as i32 - 1, col as i32),
        );
    }

    fn count_energized_for_beam(&self, beam: Beam) -> usize {
        let mut grid = MirrorGrid::new(self.rows.clone());
        grid.simulate(beam);

        return grid.count_energized();
    }

    pub fn simulate(&mut self, beam: Beam) {
        self.beams.push(beam);

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
