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
}

fn transpose<T: Copy>(vecs: &Vec<Vec<T>>) -> Vec<Vec<T>> {
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