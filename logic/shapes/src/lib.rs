use lazy_static::lazy_static;
use orientation::{Axis, Direction, Orientation, Polarity, Rotation};
use strum::{EnumCount, IntoEnumIterator};
use tetrominos::Tetromino;

mod bitfield;
mod bounding_box;
mod shape;
mod shape_id;

pub use bitfield::Bitfield;
pub use bounding_box::BoundingBox;
pub use shape::Shape;
pub use shape_id::ShapeId;

pub const MAX_SIZE: usize = 4;

lazy_static! {
    pub static ref SHAPES: [Shape; ShapeId::COUNT] = generate();
}

pub fn generate() -> [Shape; ShapeId::COUNT] {
    let arbitrary_direction = Direction::from((Polarity::Positive, Axis::X));

    let mut shapes = [Shape {
        bitboards: [0; MAX_SIZE],
        bounding_box: BoundingBox {
            start: [0; Axis::COUNT],
            end: [0; Axis::COUNT],
        },
    }; ShapeId::COUNT];

    for variant in Tetromino::iter() {
        let rotation_size = Bitfield::rotation_size(variant);
        let canonical = Bitfield::canonical(variant);

        for axis in Axis::iter() {
            for polarity in Polarity::iter() {
                let direction = Direction::from((polarity, axis));

                let facing_direction = match Direction::default().cross(direction) {
                    Some(cross) => canonical.rotate(cross, rotation_size),
                    None => {
                        if direction == Direction::default() {
                            canonical
                        } else {
                            canonical
                                .rotate(arbitrary_direction, rotation_size)
                                .rotate(arbitrary_direction, rotation_size)
                        }
                    }
                };

                for rotation in Rotation::iter() {
                    let mut orientated = facing_direction;

                    for _ in 0..rotation as usize {
                        orientated = orientated.rotate(direction, rotation_size);
                    }

                    let combination =
                        ShapeId::from((variant, Orientation::from((direction, rotation))))
                            .combination();
                    shapes[combination] = Shape {
                        bitboards: orientated.compute_bitboards(),
                        bounding_box: orientated.compute_bounding_box(),
                    };
                }
            }
        }
    }

    shapes
}
