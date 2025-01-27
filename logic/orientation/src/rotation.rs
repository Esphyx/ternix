use std::ops::Add;

use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, Default, EnumCount, EnumIter, PartialEq, Eq, Hash)]
pub enum Rotation {
    #[default]
    Identity,
    Generator,
    Double,
    Prime,
}

impl From<isize> for Rotation {
    fn from(value: isize) -> Self {
        match value.rem_euclid(Rotation::COUNT as isize) as usize {
            0 => Self::Identity,
            1 => Self::Generator,
            2 => Self::Double,
            3 => Self::Prime,
            _ => unreachable!(),
        }
    }
}

impl Add for Rotation {
    type Output = Rotation;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self as isize + rhs as isize)
    }
}

impl Rotation {
    pub fn rotate_by(&self, count: isize) -> Self {
        Self::from(*self as isize + count)
    }
}
