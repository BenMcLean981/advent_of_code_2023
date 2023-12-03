use std::collections::HashSet;

use super::{cell::Cell, number::Number, position::Position};

pub struct Grid {
    rows: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn from_lines(lines: Vec<String>) -> Self {
        let cells: Vec<Vec<Cell>> = lines
            .iter()
            .enumerate()
            .map(|(row_num, row)| Self::line_to_cells(row, row_num))
            .collect();

        return Grid { rows: cells };
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

    pub fn get_gear_ratios(&self) -> Vec<u32> {
        let numbers = self.get_numbers();
        let symbols = self.get_symbols();

        let mut gear_ratios: Vec<u32> = vec![];

        symbols.iter().filter(|s| s.char == '*').for_each(|s| {
            let nums: Vec<&Number> = numbers
                .iter()
                .filter(|n| n.is_neighboring(s.position))
                .collect();

            if nums.len() == 2 {
                gear_ratios.push(nums[0].number * nums[1].number);
            }
        });

        return gear_ratios;
    }

    pub fn get_part_numbers(&self) -> Vec<u32> {
        let numbers = self.get_numbers();
        let symbols = self.get_symbols();

        return numbers
            .iter()
            .filter(|n| symbols.iter().any(|s| n.is_neighboring(s.position)))
            .map(|pn| pn.number)
            .collect();
    }

    fn get_symbols(&self) -> HashSet<Cell> {
        let symbols = self
            .rows
            .iter()
            .flatten()
            .filter(|c| c.is_symbol())
            .map(|c| *c);

        return HashSet::from_iter(symbols);
    }

    fn get_numbers(&self) -> Vec<Number> {
        return self
            .rows
            .iter()
            .flat_map(|row| Self::get_row_numbers(row))
            .collect();
    }

    fn get_row_numbers(row: &Vec<Cell>) -> Vec<Number> {
        let mut results: Vec<Number> = vec![];
        let mut next_result: Vec<Cell> = vec![];

        let number_cells = row.iter().filter(|c| c.is_number());

        number_cells.for_each(|cell| {
            let last = next_result.last();

            if let Some(last) = last {
                if last.position.is_adjacent(cell.position) {
                    next_result.push(*cell);
                } else {
                    results.push(to_number(&next_result));
                    next_result = vec![*cell];
                }
            } else {
                next_result.push(*cell);
            }
        });

        if !next_result.is_empty() {
            results.push(to_number(&next_result));
        }

        return results;
    }
}

fn to_number(cells: &Vec<Cell>) -> Number {
    let digits = cells.iter().map(|c| c.char);
    let number = digits.collect::<String>().parse::<u32>().unwrap();

    let positions = HashSet::from_iter(cells.iter().map(|c| c.position));

    return Number::new(number, positions);
}
