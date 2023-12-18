use super::{beam::Beam, direction::Direction};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum OpticalOperator {
    None,
    HorizontalSplitter,
    VerticalSplitter,
    DownwardsMirror,
    UpwardsMirror,
}

impl OpticalOperator {
    pub fn should_split(&self, beam: &Beam) -> bool {
        return beam.direction.is_horizontal()
            && *self == OpticalOperator::VerticalSplitter
            || beam.direction.is_vertical()
                && *self == OpticalOperator::HorizontalSplitter;
    }

    pub fn split(&self, beam: &Beam) -> (Beam, Beam) {
        match self {
            OpticalOperator::HorizontalSplitter => (
                Beam::new(Direction::Left, beam.position),
                Beam::new(Direction::Right, beam.position),
            ),
            OpticalOperator::VerticalSplitter => (
                Beam::new(Direction::Up, beam.position),
                Beam::new(Direction::Down, beam.position),
            ),
            _ => panic!(),
        }
    }

    pub fn is_mirror(&self) -> bool {
        return *self == OpticalOperator::DownwardsMirror
            || *self == OpticalOperator::UpwardsMirror;
    }

    pub fn rotate(&self, beam: &Beam) -> Beam {
        match *self {
            OpticalOperator::DownwardsMirror => {
                Beam::new(beam.direction.rotate_downwards(), beam.position)
            }
            OpticalOperator::UpwardsMirror => {
                Beam::new(beam.direction.rotate_upwards(), beam.position)
            }
            _ => panic!(),
        }
    }
}

impl From<char> for OpticalOperator {
    fn from(value: char) -> Self {
        return match value {
            '.' => OpticalOperator::None,
            '|' => OpticalOperator::VerticalSplitter,
            '-' => OpticalOperator::HorizontalSplitter,
            '\\' => OpticalOperator::DownwardsMirror,
            '/' => OpticalOperator::UpwardsMirror,
            _ => panic!(),
        };
    }
}
