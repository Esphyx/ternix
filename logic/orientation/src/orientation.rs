use strum::EnumCount;

use crate::{Direction, Rotation};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq,  Hash)]
pub struct Orientation {
    pub direction: Direction,
    pub rotation: Rotation,
}

impl Orientation {
    #[inline]
    pub fn rotation_combination(&self, around: Direction) -> usize {
        around.combination() + self.combination() * Direction::COUNT
    }

    pub fn combination(&self) -> usize {
        self.rotation as usize + self.direction.combination() * Rotation::COUNT
    }
}

impl From<(Direction, Rotation)> for Orientation {
    fn from(value: (Direction, Rotation)) -> Self {
        let (direction, rotation) = value;
        Self {
            direction,
            rotation,
        }
    }
}

impl EnumCount for Orientation {
    const COUNT: usize = Direction::COUNT * Rotation::COUNT;
}
