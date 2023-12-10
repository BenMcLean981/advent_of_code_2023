use super::tile_type::TileType;

#[derive(Debug, PartialEq, Eq)]
pub struct Grid {
    rows: Vec<Vec<TileType>>,
}

impl Grid {
    pub fn new(rows: Vec<Vec<TileType>>) -> Self {
        return Grid { rows };
    }

    pub fn from_lines(lines: Vec<&str>) -> Self {
        let rows = lines
            .iter()
            .map(|l| Grid::read_line(l))
            .collect::<Vec<Vec<TileType>>>();

        return Grid::new(rows);
    }

    fn read_line(line: &str) -> Vec<TileType> {
        return line
            .chars()
            .map(|c| TileType::from(c))
            .collect::<Vec<TileType>>();
    }
}
