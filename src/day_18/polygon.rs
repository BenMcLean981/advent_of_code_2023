use super::xy::Xy;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Polygon {
    points: Vec<Xy>,
}

impl Polygon {
    pub fn new(points: Vec<Xy>) -> Self {
        let mut points = points.clone();

        if points.last().unwrap() != points.first().unwrap() {
            points.push(*points.first().unwrap());
        }

        return Polygon { points };
    }

    pub fn get_area(&self) -> u32 {
        let mut area_2 = 0;

        for i in 0..self.points.len() - 1 {
            let p1 = self.points[i];
            let p2 = self.points[i + 1];

            area_2 += p1.x * p2.y - p1.y * p2.x + 1;
        }

        return (area_2.abs() / 2) as u32;
    }
}
