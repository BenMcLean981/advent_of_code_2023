use std::collections::HashSet;

use super::{cell::Cell, position::Position};

pub struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn from_lines(lines: Vec<String>) -> Self {
        let cells: Vec<Vec<Cell>> = lines
            .iter()
            .enumerate()
            .map(|(row_num, row)| Self::line_to_cells(row, row_num))
            .collect();

        return Grid { cells };
    }

    fn line_to_cells(row: &String, row_num: usize) -> Vec<Cell> {
        let row_num = u32::try_from(row_num).unwrap();

        return row
            .chars()
            .enumerate()
            .map(|(col_num, char)| {
                let col_num = u32::try_from(col_num).unwrap();
                let position = Position::new(row_num, col_num);

                return Cell::new(position, char);
            })
            .collect();
    }

    pub fn get_part_numbers(&self) -> HashSet<u32> {
        return HashSet::<u32>::new();
    }

    pub fn get_symbols(&self) -> HashSet<&Cell> {
        let symbols = self.cells.iter().flatten().filter(|c| c.is_symbol());

        return HashSet::from_iter(symbols);
    }
}
