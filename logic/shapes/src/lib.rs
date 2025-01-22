mod bitfield;
mod bounding_box;

use std::collections::HashMap;

use lazy_static::lazy_static;
use orientation::{Axis, Orientation};
use strum::{EnumCount, IntoEnumIterator};
use tetrominos::Tetromino;

const MAX_SHAPE_SIZE: usize = 4;

lazy_static! {
    pub static ref SHAPES: HashMap<ShapeId, Shape> = generate();
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ShapeId {
    pub variant: Tetromino,
    pub orientation: Orientation,
}

pub struct Shape {
    bitboards: [u64; MAX_SHAPE_SIZE],
    dimensions: [usize; Axis::COUNT],
}


pub fn generate() -> HashMap<ShapeId, Shape> {
    for variant in Tetromino::iter() {}

    todo!()
}

pub fn base_shape_from_variant(variant: Tetromino) -> u64 {
    todo!()
}

impl Shape {
    pub fn from(shape_id: &ShapeId) -> &'static Self {
        SHAPES
            .get(shape_id)
            .expect("Could not find shape for the specified shape ID")
    }

    pub fn bitboards(&self) -> [u64; MAX_SHAPE_SIZE] {
        self.bitboards
    }

    #[inline]
    pub fn size_for(&self, axis: Axis) -> usize {
        self.dimensions[axis as usize]
    }
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
