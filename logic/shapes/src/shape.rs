use orientation::Axis;
use strum::EnumCount;

use super::{BoundingBox, ShapeId, SHAPES};

#[derive(Clone, Copy)]
pub struct Shape {
    pub bitboards: [u64; super::MAX_SIZE],
    pub bounding_box: BoundingBox,
}

impl Shape {
    pub fn size(&self) -> [usize; Axis::COUNT] {
        self.bounding_box.size()
    }
}

impl From<&ShapeId> for &'static Shape {
    fn from(value: &ShapeId) -> Self {
        &SHAPES[value.combination()]
    }
}
