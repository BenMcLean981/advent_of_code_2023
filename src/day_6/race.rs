pub struct Race {
    time: u32,
    distance: u32,
}

const ACCELERATION: f64 = 1.0;

impl Race {
    pub fn new(time: u32, distance: u32) -> Self {
        return Race { time, distance };
    }

    pub fn get_num_possible_wins(&self) -> u32 {
        match self.get_roots() {
            Roots::None => 0,
            Roots::One(_) => 1,
            Roots::Two(lower, upper) => Race::count_between(lower, upper),
        }
    }

    fn count_between(lower: f64, upper: f64) -> u32 {
        let lower = lower.ceil().round() as i32;
        let upper = upper.floor().round() as i32;

        // it's actually impossible for roots to be negative
        // when distance and time are positive, safety check anyway.
        let lower = lower.max(0) as u32;
        let upper = upper.max(0) as u32;

        return upper - lower + 1;
    }

    fn get_roots(&self) -> Roots {
        let a: f64 = ACCELERATION;
        let b: f64 = -ACCELERATION * f64::from(self.time);
        let c: f64 = self.distance.into();

        let determinant = b * b - 4.0 * a * c;

        if determinant.is_sign_negative() {
            return Roots::None;
        } else if determinant <= 1e-8 {
            let root = -b / (2.0 * a);

            return Roots::One(root);
        } else {
            let sqrt_det = determinant.sqrt();

            let root_1 = (-b - sqrt_det) / (2.0 * a);
            let root_2 = (-b + sqrt_det) / (2.0 * a);

            return Roots::Two(root_1, root_2);
        }
    }
}

enum Roots {
    None,
    One(f64),
    Two(f64, f64),
}
