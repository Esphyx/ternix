use orientation::Axis;
use strum::EnumCount;

use crate::Bitfield;

use super::{BoundingBox, ShapeId, SHAPES};

#[derive(Clone, Copy)]
pub struct Shape {
    pub bitboards: [u64; super::MAX_SIZE],
    pub bounding_box: BoundingBox,
    pub bitfield: Bitfield,
}

impl Shape {
    pub fn size(&self) -> [usize; Axis::COUNT] {
        self.bounding_box.size()
    }
}

impl From<Bitfield> for Shape {
    fn from(bitfield: Bitfield) -> Self {
        Self {
            bitboards: bitfield.compute_bitboards(),
            bounding_box: bitfield.compute_bounding_box(),
            bitfield,
        }
    }
}

impl From<&ShapeId> for &'static Shape {
    fn from(value: &ShapeId) -> Self {
        &SHAPES[value.combination()]
    }
}

#[cfg(test)]
mod tests {
    use orientation::{Axis, Direction, Orientation, Polarity, Rotation};
    use strum::EnumCount;
    use tetrominos::Tetromino;

    use crate::ShapeId;

    use super::Shape;

    #[test]
    fn size() {
        println!("{}", std::mem::size_of::<Shape>() * Orientation::COUNT);
    }

    #[test]
    fn it_works() {
        let variant = Tetromino::L;
        let shape_id = ShapeId::from((
            variant,
            Orientation::from((
                Direction::from((Polarity::Negative, Axis::Y)),
                Rotation::Prime,
            )),
        ));

        let shape: &Shape = (&shape_id).into();
        let &Shape { bitfield, .. } = shape;

        println!("{}", bitfield.to_string());
    }
}
