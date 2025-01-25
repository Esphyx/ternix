use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, Default, EnumCount, EnumIter, Eq, Hash, PartialEq)]
pub enum Axis {
    X,
    #[default]
    Y,
    Z,
}

impl Axis {
    pub fn rotate_dimensions(&self, dimensions: [usize; Axis::COUNT]) -> [usize; Axis::COUNT] {
        let [x, y, z] = dimensions;
        match self {
            Axis::X => [x, z, y],
            Axis::Y => [z, y, x],
            Axis::Z => [y, x, z],
        }
    }

    pub fn cross(&self, other: Self) -> Option<Self> {
        match (self, other) {
            (Axis::X, Axis::Y) | (Axis::Y, Axis::X) => Some(Axis::Z),
            (Axis::Y, Axis::Z) | (Axis::Z, Axis::Y) => Some(Axis::X),
            (Axis::Z, Axis::X) | (Axis::X, Axis::Z) => Some(Axis::Y),
            _ => None,
        }
    }
}
