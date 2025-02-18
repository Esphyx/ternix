use orientation::{Axis, Direction};
use shapes::{Bitfield, Shape, ShapeId};
use strum::EnumCount;
use tetrominos::Tetromino;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Piece {
    pub shape_id: ShapeId,
    pub position: [isize; Axis::COUNT],
}

impl Piece {
    pub fn rotate(&mut self, along: Direction) {
        self.shape_id = self.shape_id.rotate(along);
    }

    pub fn shape(&self) -> &'static Shape {
        (&self.shape_id).into()
    }

    pub fn translate(&mut self, offset: [isize; Axis::COUNT]) {
        let [x, y, z] = self.position;
        let [dx, dy, dz] = offset;
        self.position = [x + dx, y + dy, z + dz]
    }

    pub fn from<const W: usize, const H: usize, const D: usize>(variant: Tetromino) -> Self {
        let shape_id = ShapeId::from(variant);
        let shape: &Shape = (&shape_id).into();

        let [_, end_y, _] = shape.bounding_box.end;
        let size = Bitfield::rotation_size(variant);

        let [x, y, z] = [(W - size) / 2, H - end_y - 1, (D - size) / 2];

        Self {
            shape_id,
            position: [x as isize, y as isize, z as isize],
        }
    }
}
