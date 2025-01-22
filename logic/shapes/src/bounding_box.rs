use orientation::Axis;
use strum::EnumCount;

pub struct BoundingBox {
    pub start: [usize; Axis::COUNT],
    pub end: [usize; Axis::COUNT],
}
