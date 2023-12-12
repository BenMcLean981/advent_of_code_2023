#[derive(Debug, PartialEq, Eq)]
pub struct Universe {
    rows: Vec<Vec<bool>>,
    cols: Vec<Vec<bool>>,
}

impl Universe {
    pub fn new(rows: Vec<Vec<bool>>) -> Self {
        let cols = transpose(&rows);

        return Universe { rows, cols };
    }

    pub fn from_lines(lines: Vec<&str>) -> Self {
        let rows = lines.iter().map(|l| Universe::read_row(l)).collect();

        return Universe::new(rows);
    }

    fn read_row(line: &str) -> Vec<bool> {
        return line.trim().chars().map(|c| c == '#').collect();
    }

    pub fn expand(&self) -> Universe {
        let rows = Universe::expand_vecs(&self.rows);

        let cols = transpose(&rows);
        let cols = Universe::expand_vecs(&cols);

        let rows = transpose(&cols);

        return Universe { rows, cols };
    }

    fn expand_vecs(vecs: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
        let mut results: Vec<Vec<bool>> = vec![];

        for vec in vecs {
            if is_empty(vec) {
                results.push(make_empty(vec.len()))
            }

            results.push(vec.clone());
        }

        return results;
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

fn is_empty(vec: &Vec<bool>) -> bool {
    return vec.iter().all(|b| !b);
}

fn make_empty(size: usize) -> Vec<bool> {
    return vec![false; size];
}
