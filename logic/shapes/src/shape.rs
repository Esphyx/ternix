use orientation::{Axis, Orientation};
use strum::EnumCount;
use tetrominos::Tetromino;

use crate::SHAPES;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ShapeId {
    pub variant: Tetromino,
    pub orientation: Orientation,
}

pub struct Shape {
    bitboards: [u64; super::MAX_SIZE],
    dimensions: [usize; Axis::COUNT],
}

impl ShapeId {
    pub fn from(variant: Tetromino) -> Self {
        Self {
            variant,
            orientation: Default::default(),
        }
    }

    pub fn shape(&self) -> &Shape {
        SHAPES.get(self).expect("ShapeId key does not exist!")
    }

    pub fn rotate(&self, along: Axis) -> Self {
        todo!()
        // self.orientation.rotate();
    }
}

impl Shape {
    pub fn from(shape_id: &ShapeId) -> &'static Self {
        SHAPES
            .get(shape_id)
            .expect("Could not find shape for the specified shape ID")
    }

    pub fn bitboards(&self) -> [u64; super::MAX_SIZE] {
        self.bitboards
    }

    #[inline]
    pub fn size_for(&self, axis: Axis) -> usize {
        self.dimensions[axis as usize]
    }
}
