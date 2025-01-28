use std::time::Instant;

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
    pub static ref SHAPES: [Shape; ShapeId::COUNT] = generate_shapes();
    pub static ref ROTATIONS: [Orientation; Orientation::COUNT * Direction::COUNT] =
        generate_rotations();
}

fn generate_rotations() -> [Orientation; Orientation::COUNT * Direction::COUNT] {
    let start = Instant::now();

    let mut rotations = [Orientation::default(); Orientation::COUNT * Direction::COUNT];

    for axis in Axis::iter() {
        for polarity in Polarity::iter() {
            for rotation in Rotation::iter() {
                let orientation = Orientation::from((Direction::from((polarity, axis)), rotation));
                for axis in Axis::iter() {
                    for polarity in Polarity::iter() {
                        let around = Direction::from((polarity, axis));
                        rotations[orientation.rotation_combination(around)] =
                            rotate(orientation, around);
                    }
                }
            }
        }
    }

    println!(
        "Finished computation of rotations in {:?}!",
        start.elapsed()
    );

    rotations
}

fn rotate(orientation: Orientation, around: Direction) -> Orientation {
    

    let (polarity, axis, rotation) = match (
        orientation.direction.polarity,
        orientation.direction.axis,
        orientation.rotation,
        around.polarity,
        around.axis,
    ) {
        (Polarity::Positive, Axis::X, Rotation::Identity, Polarity::Positive, Axis::X) => {
            (Polarity::Positive, Axis::X, Rotation::Prime)
        }
        (Polarity::Positive, Axis::X, Rotation::Identity, Polarity::Positive, Axis::Y) => {
            (Polarity::Negative, Axis::Z, Rotation::Generator)
        }
        (Polarity::Positive, Axis::X, Rotation::Identity, Polarity::Positive, Axis::Z) => {
            (Polarity::Positive, Axis::Y, Rotation::Generator)
        }
        (Polarity::Positive, Axis::X, Rotation::Identity, Polarity::Negative, Axis::X) => {
            (Polarity::Positive, Axis::X, Rotation::Double)
        }
        (Polarity::Positive, Axis::X, Rotation::Identity, Polarity::Negative, Axis::Y) => {
            (Polarity::Positive, Axis::Z, Rotation::Prime)
        }
        (Polarity::Positive, Axis::X, Rotation::Identity, Polarity::Negative, Axis::Z) => {
            (Polarity::Negative, Axis::Y, Rotation::Double)
        }
        (Polarity::Positive, Axis::X, Rotation::Generator, Polarity::Positive, Axis::X) => {
            (Polarity::Positive, Axis::X, Rotation::Double)
        }
        (Polarity::Positive, Axis::X, Rotation::Generator, Polarity::Positive, Axis::Y) => {
            (Polarity::Negative, Axis::Z, Rotation::Double)
        }
        (Polarity::Positive, Axis::X, Rotation::Generator, Polarity::Positive, Axis::Z) => {
            (Polarity::Positive, Axis::Y, Rotation::Prime)
        }
        (Polarity::Positive, Axis::X, Rotation::Generator, Polarity::Negative, Axis::X) => {
            (Polarity::Positive, Axis::X, Rotation::Prime)
        }
        (Polarity::Positive, Axis::X, Rotation::Generator, Polarity::Negative, Axis::Y) => {
            (Polarity::Positive, Axis::Z, Rotation::Identity)
        }
        (Polarity::Positive, Axis::X, Rotation::Generator, Polarity::Negative, Axis::Z) => {
            (Polarity::Negative, Axis::Y, Rotation::Identity)
        }
        (Polarity::Positive, Axis::X, Rotation::Double, Polarity::Positive, Axis::X) => {
            (Polarity::Positive, Axis::X, Rotation::Identity)
        }
        (Polarity::Positive, Axis::X, Rotation::Double, Polarity::Positive, Axis::Y) => {
            (Polarity::Negative, Axis::Z, Rotation::Prime)
        }
        (Polarity::Positive, Axis::X, Rotation::Double, Polarity::Positive, Axis::Z) => {
            (Polarity::Positive, Axis::Y, Rotation::Identity)
        }
        (Polarity::Positive, Axis::X, Rotation::Double, Polarity::Negative, Axis::X) => {
            (Polarity::Positive, Axis::X, Rotation::Generator)
        }
        (Polarity::Positive, Axis::X, Rotation::Double, Polarity::Negative, Axis::Y) => {
            (Polarity::Positive, Axis::Z, Rotation::Generator)
        }
        (Polarity::Positive, Axis::X, Rotation::Double, Polarity::Negative, Axis::Z) => {
            (Polarity::Negative, Axis::Y, Rotation::Double)
        }
        (Polarity::Positive, Axis::X, Rotation::Prime, Polarity::Positive, Axis::X) => {
            (Polarity::Positive, Axis::X, Rotation::Generator)
        }
        (Polarity::Positive, Axis::X, Rotation::Prime, Polarity::Positive, Axis::Y) => {
            (Polarity::Negative, Axis::Z, Rotation::Identity)
        }
        (Polarity::Positive, Axis::X, Rotation::Prime, Polarity::Positive, Axis::Z) => {
            (Polarity::Positive, Axis::Y, Rotation::Double)
        }
        (Polarity::Positive, Axis::X, Rotation::Prime, Polarity::Negative, Axis::X) => {
            (Polarity::Positive, Axis::X, Rotation::Identity)
        }
        (Polarity::Positive, Axis::X, Rotation::Prime, Polarity::Negative, Axis::Y) => {
            (Polarity::Positive, Axis::Z, Rotation::Double)
        }
        (Polarity::Positive, Axis::X, Rotation::Prime, Polarity::Negative, Axis::Z) => {
            (Polarity::Negative, Axis::Y, Rotation::Generator)
        }
        (Polarity::Positive, Axis::Y, Rotation::Identity, Polarity::Positive, Axis::X) => {
            (Polarity::Positive, Axis::Z, Rotation::Prime)
        }
        (Polarity::Positive, Axis::Y, Rotation::Identity, Polarity::Positive, Axis::Y) => {
            (Polarity::Positive, Axis::Y, Rotation::Generator)
        }
        (Polarity::Positive, Axis::Y, Rotation::Identity, Polarity::Positive, Axis::Z) => {
            (Polarity::Negative, Axis::X, Rotation::Generator)
        }
        (Polarity::Positive, Axis::Y, Rotation::Identity, Polarity::Negative, Axis::X) => {
            (Polarity::Negative, Axis::Z, Rotation::Double)
        }
        (Polarity::Positive, Axis::Y, Rotation::Identity, Polarity::Negative, Axis::Y) => {
            (Polarity::Positive, Axis::Y, Rotation::Prime)
        }
        (Polarity::Positive, Axis::Y, Rotation::Identity, Polarity::Negative, Axis::Z) => {
            (Polarity::Positive, Axis::X, Rotation::Double)
        }
        (Polarity::Positive, Axis::Y, Rotation::Generator, Polarity::Positive, Axis::X) => {
            (Polarity::Positive, Axis::Z, Rotation::Double)
        }
        (Polarity::Positive, Axis::Y, Rotation::Generator, Polarity::Positive, Axis::Y) => {
            (Polarity::Positive, Axis::Y, Rotation::Double)
        }
        (Polarity::Positive, Axis::Y, Rotation::Generator, Polarity::Positive, Axis::Z) => {
            (Polarity::Negative, Axis::X, Rotation::Prime)
        }
        (Polarity::Positive, Axis::Y, Rotation::Generator, Polarity::Negative, Axis::X) => {
            (Polarity::Negative, Axis::Z, Rotation::Prime)
        }
        (Polarity::Positive, Axis::Y, Rotation::Generator, Polarity::Negative, Axis::Y) => {
            (Polarity::Positive, Axis::Y, Rotation::Identity)
        }
        (Polarity::Positive, Axis::Y, Rotation::Generator, Polarity::Negative, Axis::Z) => {
            (Polarity::Positive, Axis::X, Rotation::Identity)
        }
        (Polarity::Positive, Axis::Y, Rotation::Double, Polarity::Positive, Axis::X) => {
            (Polarity::Positive, Axis::Z, Rotation::Identity)
        }
        (Polarity::Positive, Axis::Y, Rotation::Double, Polarity::Positive, Axis::Y) => {
            (Polarity::Positive, Axis::Y, Rotation::Prime)
        }
        (Polarity::Positive, Axis::Y, Rotation::Double, Polarity::Positive, Axis::Z) => {
            (Polarity::Negative, Axis::X, Rotation::Identity)
        }
        (Polarity::Positive, Axis::Y, Rotation::Double, Polarity::Negative, Axis::X) => {
            (Polarity::Negative, Axis::Z, Rotation::Generator)
        }
        (Polarity::Positive, Axis::Y, Rotation::Double, Polarity::Negative, Axis::Y) => {
            (Polarity::Positive, Axis::Y, Rotation::Generator)
        }
        (Polarity::Positive, Axis::Y, Rotation::Double, Polarity::Negative, Axis::Z) => {
            (Polarity::Positive, Axis::X, Rotation::Prime)
        }
        (Polarity::Positive, Axis::Y, Rotation::Prime, Polarity::Positive, Axis::X) => {
            (Polarity::Positive, Axis::Z, Rotation::Generator)
        }
        (Polarity::Positive, Axis::Y, Rotation::Prime, Polarity::Positive, Axis::Y) => {
            (Polarity::Positive, Axis::Y, Rotation::Identity)
        }
        (Polarity::Positive, Axis::Y, Rotation::Prime, Polarity::Positive, Axis::Z) => {
            (Polarity::Negative, Axis::X, Rotation::Double)
        }
        (Polarity::Positive, Axis::Y, Rotation::Prime, Polarity::Negative, Axis::X) => {
            (Polarity::Negative, Axis::Z, Rotation::Identity)
        }
        (Polarity::Positive, Axis::Y, Rotation::Prime, Polarity::Negative, Axis::Y) => {
            (Polarity::Positive, Axis::Y, Rotation::Double)
        }
        (Polarity::Positive, Axis::Y, Rotation::Prime, Polarity::Negative, Axis::Z) => {
            (Polarity::Positive, Axis::X, Rotation::Generator)
        }
        (Polarity::Positive, Axis::Z, Rotation::Identity, Polarity::Positive, Axis::X) => {
            (Polarity::Negative, Axis::Y, Rotation::Double)
        }
        (Polarity::Positive, Axis::Z, Rotation::Identity, Polarity::Positive, Axis::Y) => {
            (Polarity::Positive, Axis::X, Rotation::Generator)
        }
        (Polarity::Positive, Axis::Z, Rotation::Identity, Polarity::Positive, Axis::Z) => {
            (Polarity::Positive, Axis::Z, Rotation::Generator)
        }
        (Polarity::Positive, Axis::Z, Rotation::Identity, Polarity::Negative, Axis::X) => {
            (Polarity::Positive, Axis::Y, Rotation::Double)
        }
        (Polarity::Positive, Axis::Z, Rotation::Identity, Polarity::Negative, Axis::Y) => {
            (Polarity::Negative, Axis::X, Rotation::Prime)
        }
        (Polarity::Positive, Axis::Z, Rotation::Identity, Polarity::Negative, Axis::Z) => {
            (Polarity::Positive, Axis::Z, Rotation::Double)
        }
        (Polarity::Positive, Axis::Z, Rotation::Generator, Polarity::Positive, Axis::X) => {
            (Polarity::Negative, Axis::Y, Rotation::Double)
        }
        (Polarity::Positive, Axis::Z, Rotation::Generator, Polarity::Positive, Axis::Y) => {
            (Polarity::Positive, Axis::X, Rotation::Double)
        }
        (Polarity::Positive, Axis::Z, Rotation::Generator, Polarity::Positive, Axis::Z) => {
            (Polarity::Positive, Axis::Z, Rotation::Prime)
        }
        (Polarity::Positive, Axis::Z, Rotation::Generator, Polarity::Negative, Axis::X) => {
            (Polarity::Positive, Axis::Y, Rotation::Prime)
        }
        (Polarity::Positive, Axis::Z, Rotation::Generator, Polarity::Negative, Axis::Y) => {
            (Polarity::Negative, Axis::X, Rotation::Identity)
        }
        (Polarity::Positive, Axis::Z, Rotation::Generator, Polarity::Negative, Axis::Z) => {
            (Polarity::Positive, Axis::Z, Rotation::Identity)
        }
        (Polarity::Positive, Axis::Z, Rotation::Double, Polarity::Positive, Axis::X) => {
            (Polarity::Negative, Axis::Y, Rotation::Identity)
        }
        (Polarity::Positive, Axis::Z, Rotation::Double, Polarity::Positive, Axis::Y) => {
            (Polarity::Positive, Axis::X, Rotation::Prime)
        }
        (Polarity::Positive, Axis::Z, Rotation::Double, Polarity::Positive, Axis::Z) => {
            (Polarity::Positive, Axis::Z, Rotation::Identity)
        }
        (Polarity::Positive, Axis::Z, Rotation::Double, Polarity::Negative, Axis::X) => {
            (Polarity::Positive, Axis::Y, Rotation::Generator)
        }
        (Polarity::Positive, Axis::Z, Rotation::Double, Polarity::Negative, Axis::Y) => {
            (Polarity::Negative, Axis::X, Rotation::Generator)
        }
        (Polarity::Positive, Axis::Z, Rotation::Double, Polarity::Negative, Axis::Z) => {
            (Polarity::Positive, Axis::Z, Rotation::Prime)
        }
        (Polarity::Positive, Axis::Z, Rotation::Prime, Polarity::Positive, Axis::X) => {
            (Polarity::Negative, Axis::Y, Rotation::Generator)
        }
        (Polarity::Positive, Axis::Z, Rotation::Prime, Polarity::Positive, Axis::Y) => {
            (Polarity::Positive, Axis::X, Rotation::Identity)
        }
        (Polarity::Positive, Axis::Z, Rotation::Prime, Polarity::Positive, Axis::Z) => {
            (Polarity::Positive, Axis::Z, Rotation::Double)
        }
        (Polarity::Positive, Axis::Z, Rotation::Prime, Polarity::Negative, Axis::X) => {
            (Polarity::Positive, Axis::Y, Rotation::Identity)
        }
        (Polarity::Positive, Axis::Z, Rotation::Prime, Polarity::Negative, Axis::Y) => {
            (Polarity::Negative, Axis::X, Rotation::Double)
        }
        (Polarity::Positive, Axis::Z, Rotation::Prime, Polarity::Negative, Axis::Z) => {
            (Polarity::Positive, Axis::Z, Rotation::Generator)
        }
        (Polarity::Negative, Axis::X, Rotation::Identity, Polarity::Positive, Axis::X) => {
            (Polarity::Negative, Axis::X, Rotation::Prime)
        }
        (Polarity::Negative, Axis::X, Rotation::Identity, Polarity::Positive, Axis::Y) => {
            (Polarity::Positive, Axis::Z, Rotation::Generator)
        }
        (Polarity::Negative, Axis::X, Rotation::Identity, Polarity::Positive, Axis::Z) => {
            (Polarity::Negative, Axis::Y, Rotation::Generator)
        }
        (Polarity::Negative, Axis::X, Rotation::Identity, Polarity::Negative, Axis::X) => {
            (Polarity::Negative, Axis::X, Rotation::Double)
        }
        (Polarity::Negative, Axis::X, Rotation::Identity, Polarity::Negative, Axis::Y) => {
            (Polarity::Negative, Axis::Z, Rotation::Prime)
        }
        (Polarity::Negative, Axis::X, Rotation::Identity, Polarity::Negative, Axis::Z) => {
            (Polarity::Positive, Axis::Y, Rotation::Double)
        }
        (Polarity::Negative, Axis::X, Rotation::Generator, Polarity::Positive, Axis::X) => {
            (Polarity::Negative, Axis::X, Rotation::Double)
        }
        (Polarity::Negative, Axis::X, Rotation::Generator, Polarity::Positive, Axis::Y) => {
            (Polarity::Positive, Axis::Z, Rotation::Double)
        }
        (Polarity::Negative, Axis::X, Rotation::Generator, Polarity::Positive, Axis::Z) => {
            (Polarity::Negative, Axis::Y, Rotation::Double)
        }
        (Polarity::Negative, Axis::X, Rotation::Generator, Polarity::Negative, Axis::X) => {
            (Polarity::Negative, Axis::X, Rotation::Prime)
        }
        (Polarity::Negative, Axis::X, Rotation::Generator, Polarity::Negative, Axis::Y) => {
            (Polarity::Negative, Axis::Z, Rotation::Identity)
        }
        (Polarity::Negative, Axis::X, Rotation::Generator, Polarity::Negative, Axis::Z) => {
            (Polarity::Positive, Axis::Y, Rotation::Identity)
        }
        (Polarity::Negative, Axis::X, Rotation::Double, Polarity::Positive, Axis::X) => {
            (Polarity::Negative, Axis::X, Rotation::Identity)
        }
        (Polarity::Negative, Axis::X, Rotation::Double, Polarity::Positive, Axis::Y) => {
            (Polarity::Positive, Axis::Z, Rotation::Prime)
        }
        (Polarity::Negative, Axis::X, Rotation::Double, Polarity::Positive, Axis::Z) => {
            (Polarity::Negative, Axis::Y, Rotation::Identity)
        }
        (Polarity::Negative, Axis::X, Rotation::Double, Polarity::Negative, Axis::X) => {
            (Polarity::Negative, Axis::X, Rotation::Generator)
        }
        (Polarity::Negative, Axis::X, Rotation::Double, Polarity::Negative, Axis::Y) => {
            (Polarity::Negative, Axis::Z, Rotation::Generator)
        }
        (Polarity::Negative, Axis::X, Rotation::Double, Polarity::Negative, Axis::Z) => {
            (Polarity::Positive, Axis::Y, Rotation::Prime)
        }
        (Polarity::Negative, Axis::X, Rotation::Prime, Polarity::Positive, Axis::X) => {
            (Polarity::Negative, Axis::X, Rotation::Generator)
        }
        (Polarity::Negative, Axis::X, Rotation::Prime, Polarity::Positive, Axis::Y) => {
            (Polarity::Positive, Axis::Z, Rotation::Identity)
        }
        (Polarity::Negative, Axis::X, Rotation::Prime, Polarity::Positive, Axis::Z) => {
            (Polarity::Negative, Axis::Y, Rotation::Double)
        }
        (Polarity::Negative, Axis::X, Rotation::Prime, Polarity::Negative, Axis::X) => {
            (Polarity::Negative, Axis::X, Rotation::Identity)
        }
        (Polarity::Negative, Axis::X, Rotation::Prime, Polarity::Negative, Axis::Y) => {
            (Polarity::Negative, Axis::Z, Rotation::Double)
        }
        (Polarity::Negative, Axis::X, Rotation::Prime, Polarity::Negative, Axis::Z) => {
            (Polarity::Positive, Axis::Y, Rotation::Generator)
        }
        (Polarity::Negative, Axis::Y, Rotation::Identity, Polarity::Positive, Axis::X) => {
            (Polarity::Negative, Axis::Z, Rotation::Prime)
        }
        (Polarity::Negative, Axis::Y, Rotation::Identity, Polarity::Positive, Axis::Y) => {
            (Polarity::Negative, Axis::Y, Rotation::Generator)
        }
        (Polarity::Negative, Axis::Y, Rotation::Identity, Polarity::Positive, Axis::Z) => {
            (Polarity::Positive, Axis::X, Rotation::Generator)
        }
        (Polarity::Negative, Axis::Y, Rotation::Identity, Polarity::Negative, Axis::X) => {
            (Polarity::Positive, Axis::Z, Rotation::Double)
        }
        (Polarity::Negative, Axis::Y, Rotation::Identity, Polarity::Negative, Axis::Y) => {
            (Polarity::Negative, Axis::Y, Rotation::Double)
        }
        (Polarity::Negative, Axis::Y, Rotation::Identity, Polarity::Negative, Axis::Z) => {
            (Polarity::Negative, Axis::X, Rotation::Double)
        }
        (Polarity::Negative, Axis::Y, Rotation::Generator, Polarity::Positive, Axis::X) => {
            (Polarity::Negative, Axis::Z, Rotation::Double)
        }
        (Polarity::Negative, Axis::Y, Rotation::Generator, Polarity::Positive, Axis::Y) => {
            (Polarity::Negative, Axis::Y, Rotation::Double)
        }
        (Polarity::Negative, Axis::Y, Rotation::Generator, Polarity::Positive, Axis::Z) => {
            (Polarity::Positive, Axis::X, Rotation::Prime)
        }
        (Polarity::Negative, Axis::Y, Rotation::Generator, Polarity::Negative, Axis::X) => {
            (Polarity::Positive, Axis::Z, Rotation::Double)
        }
        (Polarity::Negative, Axis::Y, Rotation::Generator, Polarity::Negative, Axis::Y) => {
            (Polarity::Negative, Axis::Y, Rotation::Identity)
        }
        (Polarity::Negative, Axis::Y, Rotation::Generator, Polarity::Negative, Axis::Z) => {
            (Polarity::Negative, Axis::X, Rotation::Identity)
        }
        (Polarity::Negative, Axis::Y, Rotation::Double, Polarity::Positive, Axis::X) => {
            (Polarity::Negative, Axis::Z, Rotation::Identity)
        }
        (Polarity::Negative, Axis::Y, Rotation::Double, Polarity::Positive, Axis::Y) => {
            (Polarity::Negative, Axis::Y, Rotation::Prime)
        }
        (Polarity::Negative, Axis::Y, Rotation::Double, Polarity::Positive, Axis::Z) => {
            (Polarity::Positive, Axis::X, Rotation::Identity)
        }
        (Polarity::Negative, Axis::Y, Rotation::Double, Polarity::Negative, Axis::X) => {
            (Polarity::Positive, Axis::Z, Rotation::Generator)
        }
        (Polarity::Negative, Axis::Y, Rotation::Double, Polarity::Negative, Axis::Y) => {
            (Polarity::Negative, Axis::Y, Rotation::Generator)
        }
        (Polarity::Negative, Axis::Y, Rotation::Double, Polarity::Negative, Axis::Z) => {
            (Polarity::Negative, Axis::X, Rotation::Prime)
        }
        (Polarity::Negative, Axis::Y, Rotation::Prime, Polarity::Positive, Axis::X) => {
            (Polarity::Negative, Axis::Z, Rotation::Generator)
        }
        (Polarity::Negative, Axis::Y, Rotation::Prime, Polarity::Positive, Axis::Y) => {
            (Polarity::Negative, Axis::Y, Rotation::Identity)
        }
        (Polarity::Negative, Axis::Y, Rotation::Prime, Polarity::Positive, Axis::Z) => {
            (Polarity::Positive, Axis::X, Rotation::Double)
        }
        (Polarity::Negative, Axis::Y, Rotation::Prime, Polarity::Negative, Axis::X) => {
            (Polarity::Positive, Axis::Z, Rotation::Identity)
        }
        (Polarity::Negative, Axis::Y, Rotation::Prime, Polarity::Negative, Axis::Y) => {
            (Polarity::Negative, Axis::Y, Rotation::Double)
        }
        (Polarity::Negative, Axis::Y, Rotation::Prime, Polarity::Negative, Axis::Z) => {
            (Polarity::Negative, Axis::X, Rotation::Generator)
        }
        (Polarity::Negative, Axis::Z, Rotation::Identity, Polarity::Positive, Axis::X) => {
            (Polarity::Positive, Axis::Y, Rotation::Prime)
        }
        (Polarity::Negative, Axis::Z, Rotation::Identity, Polarity::Positive, Axis::Y) => {
            (Polarity::Negative, Axis::X, Rotation::Generator)
        }
        (Polarity::Negative, Axis::Z, Rotation::Identity, Polarity::Positive, Axis::Z) => {
            (Polarity::Negative, Axis::Z, Rotation::Generator)
        }
        (Polarity::Negative, Axis::Z, Rotation::Identity, Polarity::Negative, Axis::X) => {
            (Polarity::Negative, Axis::Y, Rotation::Double)
        }
        (Polarity::Negative, Axis::Z, Rotation::Identity, Polarity::Negative, Axis::Y) => {
            (Polarity::Positive, Axis::X, Rotation::Prime)
        }
        (Polarity::Negative, Axis::Z, Rotation::Identity, Polarity::Negative, Axis::Z) => {
            (Polarity::Negative, Axis::Z, Rotation::Double)
        }
        (Polarity::Negative, Axis::Z, Rotation::Generator, Polarity::Positive, Axis::X) => {
            (Polarity::Positive, Axis::Y, Rotation::Double)
        }
        (Polarity::Negative, Axis::Z, Rotation::Generator, Polarity::Positive, Axis::Y) => {
            (Polarity::Negative, Axis::X, Rotation::Double)
        }
        (Polarity::Negative, Axis::Z, Rotation::Generator, Polarity::Positive, Axis::Z) => {
            (Polarity::Negative, Axis::Z, Rotation::Prime)
        }
        (Polarity::Negative, Axis::Z, Rotation::Generator, Polarity::Negative, Axis::X) => {
            (Polarity::Negative, Axis::Y, Rotation::Prime)
        }
        (Polarity::Negative, Axis::Z, Rotation::Generator, Polarity::Negative, Axis::Y) => {
            (Polarity::Positive, Axis::X, Rotation::Identity)
        }
        (Polarity::Negative, Axis::Z, Rotation::Generator, Polarity::Negative, Axis::Z) => {
            (Polarity::Negative, Axis::Z, Rotation::Identity)
        }
        (Polarity::Negative, Axis::Z, Rotation::Double, Polarity::Positive, Axis::X) => {
            (Polarity::Positive, Axis::Y, Rotation::Identity)
        }
        (Polarity::Negative, Axis::Z, Rotation::Double, Polarity::Positive, Axis::Y) => {
            (Polarity::Negative, Axis::X, Rotation::Prime)
        }
        (Polarity::Negative, Axis::Z, Rotation::Double, Polarity::Positive, Axis::Z) => {
            (Polarity::Negative, Axis::Z, Rotation::Identity)
        }
        (Polarity::Negative, Axis::Z, Rotation::Double, Polarity::Negative, Axis::X) => {
            (Polarity::Negative, Axis::Y, Rotation::Generator)
        }
        (Polarity::Negative, Axis::Z, Rotation::Double, Polarity::Negative, Axis::Y) => {
            (Polarity::Positive, Axis::X, Rotation::Generator)
        }
        (Polarity::Negative, Axis::Z, Rotation::Double, Polarity::Negative, Axis::Z) => {
            (Polarity::Negative, Axis::Z, Rotation::Prime)
        }
        (Polarity::Negative, Axis::Z, Rotation::Prime, Polarity::Positive, Axis::X) => {
            (Polarity::Positive, Axis::Y, Rotation::Generator)
        }
        (Polarity::Negative, Axis::Z, Rotation::Prime, Polarity::Positive, Axis::Y) => {
            (Polarity::Negative, Axis::X, Rotation::Identity)
        }
        (Polarity::Negative, Axis::Z, Rotation::Prime, Polarity::Positive, Axis::Z) => {
            (Polarity::Negative, Axis::Z, Rotation::Double)
        }
        (Polarity::Negative, Axis::Z, Rotation::Prime, Polarity::Negative, Axis::X) => {
            (Polarity::Negative, Axis::Y, Rotation::Identity)
        }
        (Polarity::Negative, Axis::Z, Rotation::Prime, Polarity::Negative, Axis::Y) => {
            (Polarity::Positive, Axis::X, Rotation::Double)
        }
        (Polarity::Negative, Axis::Z, Rotation::Prime, Polarity::Negative, Axis::Z) => {
            (Polarity::Negative, Axis::Z, Rotation::Generator)
        }
    };

    Orientation::from((Direction::from((polarity, axis)), rotation))
}

fn generate_shapes() -> [Shape; ShapeId::COUNT] {
    let start = Instant::now();

    let mut shapes = [Shape {
        bitboards: [0; MAX_SIZE],
        bounding_box: BoundingBox {
            start: [0; Axis::COUNT],
            end: [0; Axis::COUNT],
        },
        bitfield: Bitfield { value: 0 },
    }; ShapeId::COUNT];

    for variant in Tetromino::iter() {
        let rotation_size = Bitfield::rotation_size(variant);
        let canonical = Bitfield::canonical(variant);

        for polarity in Polarity::iter() {
            for axis in Axis::iter() {
                for rotation in Rotation::iter() {
                    let shape_id = ShapeId::from((
                        variant,
                        Orientation::from((Direction::from((polarity, axis)), rotation)),
                    ));

                    let directions = match (polarity, axis, rotation) {
                        (Polarity::Positive, Axis::X, Rotation::Identity) => {
                            vec![(Polarity::Positive, Axis::Y), (Polarity::Negative, Axis::Z)]
                        }
                        (Polarity::Positive, Axis::X, Rotation::Generator) => {
                            vec![(Polarity::Negative, Axis::Z), (Polarity::Negative, Axis::X)]
                        }
                        (Polarity::Positive, Axis::X, Rotation::Double) => {
                            vec![(Polarity::Negative, Axis::Z)]
                        }
                        (Polarity::Positive, Axis::X, Rotation::Prime) => vec![
                            (Polarity::Positive, Axis::Y),
                            (Polarity::Negative, Axis::Z),
                            (Polarity::Positive, Axis::X),
                        ],
                        (Polarity::Positive, Axis::Y, Rotation::Identity) => Vec::new(),
                        (Polarity::Positive, Axis::Y, Rotation::Generator) => {
                            vec![(Polarity::Positive, Axis::Y)]
                        }
                        (Polarity::Positive, Axis::Y, Rotation::Double) => {
                            vec![(Polarity::Positive, Axis::Y), (Polarity::Positive, Axis::Y)]
                        }
                        (Polarity::Positive, Axis::Y, Rotation::Prime) => {
                            vec![(Polarity::Negative, Axis::Y)]
                        }
                        (Polarity::Positive, Axis::Z, Rotation::Identity) => {
                            vec![
                                (Polarity::Negative, Axis::Z),
                                (Polarity::Negative, Axis::Y),
                                (Polarity::Negative, Axis::Z),
                            ]
                        }
                        (Polarity::Positive, Axis::Z, Rotation::Generator) => {
                            vec![(Polarity::Positive, Axis::X), (Polarity::Negative, Axis::Z)]
                        }
                        (Polarity::Positive, Axis::Z, Rotation::Double) => {
                            vec![(Polarity::Positive, Axis::X), (Polarity::Positive, Axis::Z)]
                        }
                        (Polarity::Positive, Axis::Z, Rotation::Prime) => {
                            vec![(Polarity::Positive, Axis::X)]
                        }
                        (Polarity::Negative, Axis::X, Rotation::Identity) => {
                            vec![
                                (Polarity::Negative, Axis::Y),
                                (Polarity::Positive, Axis::Z),
                                (Polarity::Positive, Axis::X),
                            ]
                        }
                        (Polarity::Negative, Axis::X, Rotation::Generator) => {
                            vec![(Polarity::Positive, Axis::Z)]
                        }
                        (Polarity::Negative, Axis::X, Rotation::Double) => {
                            vec![(Polarity::Negative, Axis::Y), (Polarity::Positive, Axis::Z)]
                        }
                        (Polarity::Negative, Axis::X, Rotation::Prime) => {
                            vec![(Polarity::Positive, Axis::Z), (Polarity::Negative, Axis::X)]
                        }
                        (Polarity::Negative, Axis::Y, Rotation::Identity) => vec![
                            (Polarity::Positive, Axis::X),
                            (Polarity::Negative, Axis::Y),
                            (Polarity::Positive, Axis::Z),
                        ],
                        (Polarity::Negative, Axis::Y, Rotation::Generator) => {
                            vec![(Polarity::Positive, Axis::X), (Polarity::Positive, Axis::X)]
                        }
                        (Polarity::Negative, Axis::Y, Rotation::Double) => vec![
                            (Polarity::Negative, Axis::Z),
                            (Polarity::Positive, Axis::X),
                            (Polarity::Negative, Axis::Z),
                        ],
                        (Polarity::Negative, Axis::Y, Rotation::Prime) => {
                            vec![(Polarity::Positive, Axis::Z), (Polarity::Positive, Axis::Z)]
                        }
                        (Polarity::Negative, Axis::Z, Rotation::Identity) => {
                            vec![(Polarity::Negative, Axis::X), (Polarity::Positive, Axis::Z)]
                        }
                        (Polarity::Negative, Axis::Z, Rotation::Generator) => {
                            vec![
                                (Polarity::Positive, Axis::Y),
                                (Polarity::Negative, Axis::X),
                                (Polarity::Negative, Axis::Z),
                            ]
                        }
                        (Polarity::Negative, Axis::Z, Rotation::Double) => {
                            vec![(Polarity::Negative, Axis::X)]
                        }
                        (Polarity::Negative, Axis::Z, Rotation::Prime) => {
                            vec![(Polarity::Negative, Axis::X), (Polarity::Negative, Axis::Z)]
                        }
                    };

                    let mut orientated = canonical;
                    for &pair in directions.iter() {
                        orientated = orientated.rotate(Direction::from(pair), rotation_size);
                    }

                    shapes[shape_id.combination()] = Shape::from(orientated);
                }
            }
        }
    }

    println!("Finished computation of shapes in {:?}", start.elapsed());

    shapes
}
