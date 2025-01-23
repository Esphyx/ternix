mod bitfield;
mod bounding_box;
mod shape;

use std::collections::HashMap;

use lazy_static::lazy_static;
use shape::{Shape, ShapeId};
use tetrominos::Tetromino;

const MAX_SIZE: usize = 4;

lazy_static! {
    pub static ref SHAPES: HashMap<ShapeId, Shape> = generate();
}

pub fn generate() -> HashMap<ShapeId, Shape> {
    todo!()
}

pub fn base_shape_from_variant(_variant: Tetromino) -> u64 {
    todo!()
}
