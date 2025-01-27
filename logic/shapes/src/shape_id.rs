use orientation::{Direction, Orientation};
use strum::EnumCount;
use tetrominos::Tetromino;

use crate::ROTATIONS;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ShapeId {
    pub variant: Tetromino,
    pub orientation: Orientation,
}

impl ShapeId {
    pub fn rotate(&self, around: Direction) -> Self {
        Self {
            variant: self.variant,
            orientation: ROTATIONS[self.orientation.rotation_combination(around)],
        }
    }

    pub fn combination(&self) -> usize {
        self.orientation.combination() + self.variant as usize * Orientation::COUNT
    }
}

impl From<Tetromino> for ShapeId {
    fn from(variant: Tetromino) -> Self {
        Self {
            variant,
            orientation: Default::default(),
        }
    }
}

impl From<(Tetromino, Orientation)> for ShapeId {
    fn from(parameters: (Tetromino, Orientation)) -> Self {
        let (variant, orientation) = parameters;
        Self {
            variant,
            orientation,
        }
    }
}

impl EnumCount for ShapeId {
    const COUNT: usize = Tetromino::COUNT * Orientation::COUNT;
}
