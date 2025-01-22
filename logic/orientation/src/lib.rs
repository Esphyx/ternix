use std::ops::Sub;

use strum::EnumCount;
use strum_macros::EnumCount;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Orientation {
    direction: Direction,
    rotation: Rotation,
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Direction {
    pub polarity: Polarity,
    pub axis: Axis,
}

#[derive(Clone, Copy, Debug, Default, EnumCount, Hash, PartialEq, Eq)]
pub enum Polarity {
    #[default]
    Positive,
    Negative,
}

#[derive(Clone, Copy, Debug, Default, EnumCount, Hash, PartialEq, Eq)]
pub enum Axis {
    X,
    #[default]
    Y,
    Z,
}

#[derive(Clone, Copy, Debug, Default, EnumCount, Hash, PartialEq, Eq)]
pub enum Rotation {
    #[default]
    Identity,
    Generator,
    Double,
    Prime,
}

impl Orientation {
    pub fn rotate(&self, along: Direction) -> Self {
        todo!()
    }

    pub fn combination(&self) -> usize {
        self.rotation as usize + self.direction.combination() * Rotation::COUNT
    }

    #[inline]
    pub const fn count() -> usize {
        Direction::count() * Rotation::COUNT
    }
}

impl Direction {
    pub fn rotate_coordinate(
        &self,
        coordinate: [usize; Axis::COUNT],
        original_dimensions: [usize; Axis::COUNT],
    ) -> [usize; Axis::COUNT] {
        let Self { axis, polarity } = self;

        let [x, y, z] = coordinate;
        let [x_size, y_size, z_size] = original_dimensions;

        // rotates counter clockwise while looking in the direction
        match (axis, polarity) {
            (Axis::X, Polarity::Positive) => [x, y_size - z - 1, y],
            (Axis::X, Polarity::Negative) => [x, z, z_size - y - 1],
            (Axis::Y, Polarity::Positive) => [x_size - z - 1, y, x],
            (Axis::Y, Polarity::Negative) => [z, y, z_size - x - 1],
            (Axis::Z, Polarity::Positive) => [x_size - y - 1, x, z],
            (Axis::Z, Polarity::Negative) => [y, y_size - x - 1, z],
        }
    }

    pub fn combination(&self) -> usize {
        self.polarity as usize + self.axis as usize * Polarity::COUNT
    }

    #[inline]
    pub const fn count() -> usize {
        Polarity::COUNT * Axis::COUNT
    }
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

    pub fn cross(&self, other: Self) -> Self {
        match (self, other) {
            (Axis::X, Axis::Y) | (Axis::Y, Axis::X) => Axis::Z,
            (Axis::Y, Axis::Z) | (Axis::Z, Axis::Y) => Axis::X,
            (Axis::Z, Axis::X) | (Axis::X, Axis::Z) => Axis::Y,
            _ => unreachable!(),
        }
    }
}
