use num_bigfloat::BigFloat;

pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn new(time: u64, distance: u64) -> Self {
        return Race { time, distance };
    }

    pub fn get_num_possible_wins(&self) -> u64 {
        match self.get_roots() {
            Roots::None => 0,
            Roots::One(_) => 1,
            Roots::Two(lower, upper) => Race::count_between(lower, upper),
        }
    }

    fn count_between(lower: BigFloat, upper: BigFloat) -> u64 {
        let both_beat_exactly = is_integer(lower) && is_integer(upper);
        let neither_beats_exactly = !is_integer(lower) && !is_integer(upper);

        let lower = lower.ceil().to_i64().unwrap();
        let upper = upper.floor().to_i64().unwrap();

        // it's actually impossible for roots to be negative
        // when distance and time are positive, safety check anyway.
        let lower = lower.max(0) as u64;
        let upper = upper.max(0) as u64;

        if both_beat_exactly {
            return upper - lower - 1;
        } else if neither_beats_exactly {
            return upper - lower + 1;
        } else {
            // one must beat exactly
            return upper - lower;
        }
    }

    fn get_roots(&self) -> Roots {
        let acceleration = BigFloat::from(1);

        let a: BigFloat = acceleration;
        let b: BigFloat = -acceleration * BigFloat::from(self.time);
        let c: BigFloat = self.distance.into();

        let determinant: BigFloat = b * b - BigFloat::from(4.0) * a * c;

        if determinant.is_negative() {
            return Roots::None;
        } else if determinant <= (1e-8).into() {
            let root: BigFloat = -b / (BigFloat::from(2.0) * a);

            return Roots::One(root);
        } else {
            let sqrt_det = determinant.sqrt();

            let root_1: BigFloat = (-b - sqrt_det) / (BigFloat::from(2.0) * a);
            let root_2: BigFloat = (-b + sqrt_det) / (BigFloat::from(2.0) * a);

            return Roots::Two(root_1, root_2);
        }
    }
}

enum Roots {
    None,
    One(BigFloat),
    Two(BigFloat, BigFloat),
}

fn is_integer(f: BigFloat) -> bool {
    return f.ceil() == f.floor();
}
