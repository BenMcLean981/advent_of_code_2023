use std::{cmp, collections::HashSet};

use crate::day_10::position::Position;

#[derive(Debug, PartialEq, Eq)]
pub struct Universe {
    rows: Vec<Vec<bool>>,
    cols: Vec<Vec<bool>>,
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
}

impl Universe {
    pub fn new(rows: Vec<Vec<bool>>) -> Self {
        let cols = transpose(&rows);

        return Universe {
            empty_rows: map_empty(&rows),
            empty_cols: map_empty(&cols),
            rows,
            cols,
        };
    }

    pub fn from_lines(lines: Vec<&str>) -> Self {
        let rows = lines.iter().map(|l| Universe::read_row(l)).collect();

        return Universe::new(rows);
    }

    fn read_row(line: &str) -> Vec<bool> {
        return line.trim().chars().map(|c| c == '#').collect();
    }

    pub fn get_sum_distances(&self, expansion: u32) -> u64 {
        let galaxies = self.get_galaxies();

        let pairs = get_pairs(galaxies.iter());

        return pairs
            .iter()
            .map(|p| self.get_manhattan_distance(p.0, p.1, expansion))
            .fold(0, |sum, i| sum + i);
    }

    fn get_manhattan_distance(
        &self,
        p1: &Position,
        p2: &Position,
        expansion: u32,
    ) -> u64 {
        let empty_rows =
            self.count_empty_rows(p1.row as usize, p2.row as usize);
        let empty_cols =
            self.count_empty_cols(p1.col as usize, p2.col as usize);

        let row_diff = p1.row.abs_diff(p2.row);
        let col_diff = p1.col.abs_diff(p2.col);

        return row_diff as u64
            + col_diff as u64
            + empty_rows as u64 * (expansion as u64 - 1)
            + empty_cols as u64 * (expansion as u64 - 1);
    }

    fn count_empty_rows(&self, start: usize, end: usize) -> usize {
        return Universe::count_empty(&self.empty_rows, start, end);
    }

    fn count_empty_cols(&self, start: usize, end: usize) -> usize {
        return Universe::count_empty(&self.empty_cols, start, end);
    }

    fn count_empty(set: &HashSet<usize>, start: usize, end: usize) -> usize {
        let low = cmp::min(start, end);
        let high = cmp::max(start, end);

        // there's another way to do this but it's slower, you can loop over
        // [low..high] and check for in set, but that size is larger than
        // size of set, it is proportional to distance.
        return set.iter().filter(|v| low < **v && **v < high).count();
    }

    fn get_galaxies(&self) -> HashSet<Position> {
        let mut result = HashSet::<Position>::new();

        for (i, row) in self.rows.iter().enumerate() {
            for (j, is_galaxy) in row.iter().enumerate() {
                if *is_galaxy {
                    let position = Position::new(i as i32, j as i32);
                    result.insert(position);
                }
            }
        }

        return result;
    }
}

pub fn transpose<T: Copy>(vecs: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = vec![];

    for row in vecs {
        for (i, col) in row.iter().enumerate() {
            if result.get(i).is_none() {
                result.push(vec![*col]);
            } else {
                result[i].push(*col);
            }
        }
    }

    return result;
}

fn get_pairs<T: Copy>(items: impl Iterator<Item = T>) -> Vec<(T, T)> {
    let mut result: Vec<(T, T)> = vec![];

    let items = &items.collect::<Vec<T>>();

    for (i, item1) in items.clone().iter().enumerate() {
        for item2 in items.iter().skip(i + 1) {
            result.push((*item1, *item2));
        }
    }

    return result;
}

fn map_empty(vecs: &Vec<Vec<bool>>) -> HashSet<usize> {
    return vecs
        .iter()
        .enumerate()
        .filter(|(_, vec)| is_empty(vec))
        .map(|(i, _)| i)
        .collect::<HashSet<usize>>();
}

fn is_empty(vec: &Vec<bool>) -> bool {
    return vec.iter().all(|b| !b);
}
