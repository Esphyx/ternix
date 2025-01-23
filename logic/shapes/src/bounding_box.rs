use orientation::{Axis, Direction};
use strum::EnumCount;

pub struct BoundingBox {
    pub start: [usize; Axis::COUNT],
    pub end: [usize; Axis::COUNT],
}

impl BoundingBox {
    pub fn rotate(&self, along: Direction) -> Self {
        let s = super::MAX_SIZE;

        let rotated_start = along.rotate_coordinate(self.start, self.size());
        let rotated_end = along.rotate_coordinate(self.end, self.size());

        let mut start = [usize::MAX; Axis::COUNT];
        let mut end = [usize::MIN; Axis::COUNT];

        for axis in 0..Axis::COUNT {
            start[axis] = rotated_start[axis].min(rotated_end[axis]);
            end[axis] = rotated_start[axis].max(rotated_end[axis]);
        }

        Self { start, end }
    }

    pub fn size(&self) -> [usize; Axis::COUNT] {
        let [start_x, start_y, start_z] = self.start;
        let [end_x, end_y, end_z] = self.end;

        [end_x - start_x, end_y - start_y, end_z - start_z]
    }
}
