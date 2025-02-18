use strum::EnumCount;

use super::{axis::Axis, polarity::Polarity};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Direction {
    pub polarity: Polarity,
    pub axis: Axis,
}

impl Direction {
    pub fn cross(&self, other: Self) -> Option<Self> {
        Some(Self {
            polarity: self.polarity.cross(other.polarity),
            axis: self.axis.cross(other.axis)?,
        })
    }

    pub fn opposite(&self) -> Self {
        Self {
            axis: self.axis,
            polarity: self.polarity.opposite(),
        }
    }

    pub fn rotate_coordinate(
        &self,
        coordinate: [usize; Axis::COUNT],
        size: usize,
    ) -> [usize; Axis::COUNT] {
        let Self { axis, polarity } = self;

        let [x, y, z] = coordinate;

        // externally clockwise
        match (axis, polarity) {
            (Axis::X, Polarity::Positive) => [x, size - z - 1, y],
            (Axis::X, Polarity::Negative) => [x, z, size - y - 1],
            (Axis::Y, Polarity::Positive) => [z, y, size - x - 1],
            (Axis::Y, Polarity::Negative) => [size - z - 1, y, x],
            (Axis::Z, Polarity::Positive) => [size - y - 1, x, z],
            (Axis::Z, Polarity::Negative) => [y, size - x - 1, z],
        }
    }

    pub fn combination(&self) -> usize {
        self.polarity as usize + self.axis as usize * Polarity::COUNT
    }
}

impl From<(Polarity, Axis)> for Direction {
    fn from(parameters: (Polarity, Axis)) -> Self {
        let (polarity, axis) = parameters;
        Self { polarity, axis }
    }
}

impl Into<(Polarity, Axis)> for Direction {
    fn into(self) -> (Polarity, Axis) {
        (self.polarity, self.axis)
    }
}

impl EnumCount for Direction {
    const COUNT: usize = Polarity::COUNT * Axis::COUNT;
}

#[cfg(test)]
mod tests {
    use super::{Axis, Direction, Polarity};

    #[test]
    fn it_works() {
        let a = Direction {
            axis: Axis::X,
            polarity: Polarity::Positive,
        };
        let b = Direction {
            axis: Axis::Z,
            polarity: Polarity::Positive,
        };

        let c = a.cross(b).unwrap();

        assert_eq!(
            c,
            Direction {
                axis: Axis::Y,
                polarity: Polarity::Positive
            }
        )
    }
}
