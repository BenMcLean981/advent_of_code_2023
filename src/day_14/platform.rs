use crate::day_11::universe::transpose;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Platform {
    rows: Vec<Vec<Rock>>,
    cols: Vec<Vec<Rock>>,
}

impl Platform {
    pub fn new(rows: Vec<Vec<Rock>>) -> Self {
        let cols = transpose(&rows);

        return Platform { rows, cols };
    }

    pub fn from_lines(lines: Vec<&str>) -> Self {
        let rows = lines
            .iter()
            .map(|l| read_line(l))
            .collect::<Vec<Vec<Rock>>>();

        fn read_line(l: &str) -> Vec<Rock> {
            return l.chars().map(Rock::from).collect();
        }

        return Platform::new(rows);
    }

    pub fn spin(&self) -> Self {
        let north = self.tilt_north();
        let west = north.tilt_west();
        let south = west.tilt_south();

        return south.tilt_east();
    }

    pub fn tilt_north(&self) -> Self {
        let cols = self
            .cols
            .iter()
            .map(|c| Platform::tilt_col_north(c))
            .collect::<Vec<Vec<Rock>>>();

        let rows = transpose(&cols);

        return Platform::new(rows);
    }

    fn tilt_col_north(col: &Vec<Rock>) -> Vec<Rock> {
        let mut new_col = col.clone();

        for (i, c) in col.iter().enumerate() {
            if *c == Rock::Round {
                let target = Platform::find_next_empty_north(&new_col, i);

                if target.is_some() {
                    new_col[target.unwrap()] = *c;
                    new_col[i] = Rock::None;
                }
            }
        }

        return new_col;
    }

    fn find_next_empty_north(col: &Vec<Rock>, from: usize) -> Option<usize> {
        let mut result: Option<usize> = None;

        let previous = col.iter().take(from).rev().enumerate();

        for (i, c) in previous {
            if *c != Rock::None {
                return result;
            }

            result = Some(from - i - 1);
        }

        return result;
    }

    pub fn tilt_west(&self) -> Self {
        let rows = self
            .rows
            .iter()
            .map(|r| Platform::tilt_row_west(r))
            .collect::<Vec<Vec<Rock>>>();

        return Platform::new(rows);
    }

    fn tilt_row_west(row: &Vec<Rock>) -> Vec<Rock> {
        let mut new_row = row.clone();

        for (i, c) in row.iter().enumerate() {
            if *c == Rock::Round {
                let target = Platform::find_next_empty_west(&new_row, i);

                if target.is_some() {
                    new_row[target.unwrap()] = *c;
                    new_row[i] = Rock::None;
                }
            }
        }

        return new_row;
    }

    fn find_next_empty_west(row: &Vec<Rock>, from: usize) -> Option<usize> {
        let mut result: Option<usize> = None;

        let previous = row.iter().take(from).rev().enumerate();

        for (i, c) in previous {
            if *c != Rock::None {
                return result;
            }

            result = Some(from - i - 1);
        }

        return result;
    }

    pub fn tilt_south(&self) -> Self {
        let cols = self
            .cols
            .iter()
            .map(|c| Platform::tilt_col_south(c))
            .collect::<Vec<Vec<Rock>>>();

        let rows = transpose(&cols);

        return Platform::new(rows);
    }

    fn tilt_col_south(col: &Vec<Rock>) -> Vec<Rock> {
        let reversed = col.iter().rev().copied().collect::<Vec<Rock>>();

        return Platform::tilt_col_north(&reversed)
            .iter()
            .rev()
            .copied()
            .collect();
    }

    pub fn tilt_east(&self) -> Self {
        let rows = self
            .rows
            .iter()
            .map(|c| Platform::tilt_row_east(c))
            .collect::<Vec<Vec<Rock>>>();

        return Platform::new(rows);
    }

    fn tilt_row_east(row: &Vec<Rock>) -> Vec<Rock> {
        let reversed = row.iter().rev().copied().collect::<Vec<Rock>>();

        return Platform::tilt_row_west(&reversed)
            .iter()
            .rev()
            .copied()
            .collect();
    }

    pub fn compute_north_load(&self) -> usize {
        return self
            .rows
            .iter()
            .enumerate()
            .map(|(i, row)| Platform::compute_row_load_north(&self, row, i))
            .sum();
    }

    fn compute_row_load_north(&self, row: &Vec<Rock>, i: usize) -> usize {
        let num_rocks = row.iter().filter(|r| **r == Rock::Round).count();

        let dist = self.rows.len() - i;

        return num_rocks * dist;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Rock {
    None,
    Cube,
    Round,
}

impl From<char> for Rock {
    fn from(value: char) -> Self {
        return match value {
            '.' => Rock::None,
            '#' => Rock::Cube,
            'O' => Rock::Round,
            _ => panic!(),
        };
    }
}
