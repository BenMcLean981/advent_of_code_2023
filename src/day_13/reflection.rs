use std::cmp;

use crate::day_11::universe::transpose;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Reflection {
    rows: Vec<Vec<Item>>,
    cols: Vec<Vec<Item>>,
}

/**
 * To match the problem, cols and rows are 1 indexed.
 */

impl Reflection {
    pub fn new(rows: Vec<Vec<Item>>) -> Self {
        let cols = transpose(&rows);

        return Reflection { rows, cols };
    }

    pub fn from_lines(lines: Vec<&str>) -> Self {
        let rows = lines.iter().map(|l| read_line(l)).collect();

        fn read_line(l: &str) -> Vec<Item> {
            return l.chars().map(Item::from).collect();
        }

        return Reflection::new(rows);
    }

    pub fn find_mirror(&self) -> Option<Mirror> {
        let col = self.find_mirrored_col();
        let row = self.find_mirrored_row();

        if col.is_some() {
            return Some(Mirror::Vertical(col.unwrap()));
        } else if row.is_some() {
            return Some(Mirror::Horizontal(row.unwrap()));
        } else {
            return None;
        }
    }

    pub fn find_mirror_with_override(
        &self,
        wrong_answer: &Mirror,
    ) -> Option<Mirror> {
        let col = self.find_mirrored_col_with_override(wrong_answer);
        let row = self.find_mirrored_row_with_override(wrong_answer);

        if col.is_some() {
            return Some(Mirror::Vertical(col.unwrap()));
        } else if row.is_some() {
            return Some(Mirror::Horizontal(row.unwrap()));
        } else {
            return None;
        }
    }

    fn find_mirrored_col_with_override(
        &self,
        wrong_answer: &Mirror,
    ) -> Option<usize> {
        return (1..=self.cols.len()).find(|col| {
            self.is_mirror_after_col(*col)
                && Mirror::Vertical(*col) != *wrong_answer
        });
    }

    pub fn find_mirrored_col(&self) -> Option<usize> {
        return (1..=self.cols.len())
            .find(|col| self.is_mirror_after_col(*col));
    }

    pub fn is_mirror_after_col(&self, col: usize) -> bool {
        if col == self.cols.len() {
            return false;
        }

        let max_offset = cmp::min(col - 1, self.cols.len() - col - 1);

        return (0..=max_offset).all(|offset| {
            self.are_cols_mirroed(col - offset, col + offset + 1)
        });
    }

    pub fn are_cols_mirroed(&self, col1: usize, col2: usize) -> bool {
        return self.cols[col1 - 1] == self.cols[col2 - 1];
    }

    fn find_mirrored_row_with_override(
        &self,
        wrong_answer: &Mirror,
    ) -> Option<usize> {
        return (1..=self.rows.len()).find(|row| {
            self.is_mirror_after_row(*row)
                && Mirror::Horizontal(*row) != *wrong_answer
        });
    }

    pub fn find_mirrored_row(&self) -> Option<usize> {
        return (1..=self.rows.len())
            .find(|row| self.is_mirror_after_row(*row));
    }

    pub fn is_mirror_after_row(&self, row: usize) -> bool {
        if row == self.rows.len() {
            return false;
        }

        let max_offset = cmp::min(row - 1, self.rows.len() - row - 1);

        return (0..=max_offset).all(|offset| {
            self.are_rows_mirroed(row - offset, row + offset + 1)
        });
    }

    pub fn are_rows_mirroed(&self, row1: usize, row2: usize) -> bool {
        return self.rows[row1 - 1] == self.rows[row2 - 1];
    }

    pub fn make_alternatives(&self) -> Vec<Reflection> {
        let mut results: Vec<Reflection> = vec![];

        for i in 1..=self.rows.len() {
            for j in 1..=self.cols.len() {
                results.push(self.flip(i, j))
            }
        }

        return results;
    }

    fn flip(&self, row: usize, col: usize) -> Self {
        let mut rows = self.rows.clone();
        let current = self.rows[row - 1][col - 1];

        rows[row - 1][col - 1] = current.flip();

        return Reflection::new(rows);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Item {
    Ash,
    Rock,
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '.' => Item::Ash,
            '#' => Item::Rock,
            _ => panic!(),
        }
    }
}

impl Item {
    pub fn flip(&self) -> Self {
        match self {
            Item::Ash => Item::Rock,
            Item::Rock => Item::Ash,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mirror {
    Horizontal(usize),
    Vertical(usize),
}
