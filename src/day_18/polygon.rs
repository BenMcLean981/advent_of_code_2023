use super::xy::Xy;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Polygon {
    points: Vec<Xy>,
}

impl Polygon {
    pub fn new(points: Vec<Xy>) -> Self {
        return Polygon { points };
    }

    pub fn get_area(&self) -> u64 {
        let mut area_2 = 0;

        for i in 0..self.points.len() - 1 {
            let p1 = self.points[i];
            let p2 = self.points[i + 1];

            area_2 += p1.x * p2.y - p1.y * p2.x;
        }

        return area_2.abs() as u64 / 2 + self.get_perimeter() / 2 + 1;
    }

    pub fn get_perimeter(&self) -> u64 {
        let mut result = 0;

        for i in 0..self.points.len() - 1 {
            let p1 = self.points[i];
            let p2 = self.points[i + 1];

            result += p1.abs_diff(&p2);
        }

        return result;
    }
}
