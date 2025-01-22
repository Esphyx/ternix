use orientation::{Axis, Orientation};
use shapes::{Shape, ShapeId};
use strum::EnumCount;
use tetrominos::Tetromino;

#[derive(Debug)]
pub struct Piece {
    pub shape_id: ShapeId,
    pub position: [usize; Axis::COUNT],
}

impl Piece {
    pub fn from<const W: usize, const H: usize, const D: usize>(variant: Tetromino) -> Self {
        let shape_id = ShapeId::from(variant);
        // let shape = Shape::from(&shape_id);

        // let position = [
        //     (W + shape.size_for(Axis::X)) / 2,
        //     H,
        //     (D + shape.size_for(Axis::Z)) / 2,
        // ];

        let position = [0, 0, 0];

        Self { shape_id, position }
    }
}
