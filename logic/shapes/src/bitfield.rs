use colored::Colorize;
use orientation::{Axis, Direction};
use strum::EnumCount;
use tetrominos::Tetromino;

use crate::bounding_box::BoundingBox;

#[derive(Clone, Copy)]
pub struct Bitfield {
    pub value: u64,
}

const FIELD_SIZE: usize = super::MAX_SIZE;
impl Bitfield {
    #[inline]
    pub fn index(x: usize, y: usize, z: usize) -> usize {
        x + y * FIELD_SIZE * FIELD_SIZE + z * FIELD_SIZE
    }

    #[inline]
    pub fn is_filled(&self, x: usize, y: usize, z: usize) -> bool {
        (self.value >> Self::index(x, y, z)) & 1 == 1
    }

    pub fn canonical(variant: Tetromino) -> Self {
        // for consistency in the canonical representation (pos y):
        // the projecting part of the shape always tries to be as far away from the origin
        // and the x axis is the preffered axis for the "body" of the shape
        match variant {
            Tetromino::I => Self { value: 0b0000_0000_0000_0000__0010_0010_0010_0010__0000_0000_0000_0000__0000_0000_0000_0000 },
            Tetromino::O => Self { value: 0b0000_0000_0000_0000__0000_0000_0000_0000__0000_0000_0011_0011__0000_0000_0000_0000 },
            Tetromino::T => Self { value: 0b0000_0000_0000_0000__0000_0000_0010_0000__0000_0000_0111_0000__0000_0000_0000_0000 },
            Tetromino::L => Self { value: 0b0000_0000_0000_0000__0000_0000_0100_0000__0000_0000_0111_0000__0000_0000_0000_0000 },
            Tetromino::S => Self { value: 0b0000_0000_0000_0000__0000_0000_0110_0000__0000_0000_0011_0000__0000_0000_0000_0000 },
            Tetromino::B => Self { value: 0b0000_0000_0000_0000__0000_0000_0000_0000__0000_0000_0010_0000__0000_0000_0011_0010 },
            Tetromino::D => Self { value: 0b0000_0000_0000_0000__0000_0000_0000_0000__0000_0000_0010_0000__0000_0000_0010_0011 },
            Tetromino::F => Self { value: 0b0000_0000_0000_0000__0000_0000_0000_0000__0000_0000_0010_0000__0000_0000_0011_0001 },
        }
    }

    pub fn rotation_size(variant: Tetromino) -> usize {
        match variant {
            Tetromino::I => 4,
            Tetromino::O => 2,
            Tetromino::T => 3,
            Tetromino::L => 3,
            Tetromino::S => 3,
            Tetromino::B => 2,
            Tetromino::D => 2,
            Tetromino::F => 2,
        }
    }

    pub fn rotate(&self, along: Direction, size: usize) -> Self {
        let mut value = 0;

        for y in 0..FIELD_SIZE {
            for z in 0..FIELD_SIZE {
                for x in 0..FIELD_SIZE {
                    if self.is_filled(x, y, z) {
                        let [i, j, k] = along.rotate_coordinate([x, y, z], size);
                        value |= 1 << Self::index(i, j, k);
                    }
                }
            }
        }

        Self { value }
    }

    pub fn compute_bitboards(&self) -> [u64; FIELD_SIZE] {
        let mut layers = [0; FIELD_SIZE];

        for y in 0..FIELD_SIZE {
            let mut layer = 0;
            for z in 0..FIELD_SIZE {
                for x in 0..FIELD_SIZE {
                    if self.is_filled(x, y, z) {
                        let index = x + z * layer::MAX_SIZE;
                        layer |= 1 << index;
                    }
                }
            }
            layers[y] = layer;
        }

        layers
    }

    pub fn compute_bounding_box(&self) -> BoundingBox {
        let mut active_bits = Vec::new();

        for y in 0..FIELD_SIZE {
            for z in 0..FIELD_SIZE {
                for x in 0..FIELD_SIZE {
                    if self.is_filled(x, y, z) {
                        active_bits.push([x, y, z]);
                    }
                }
            }
        }

        let mut start = [FIELD_SIZE; Axis::COUNT];
        let mut end = [0; Axis::COUNT];

        for bit_coordinate in active_bits {
            for (axis, &component) in bit_coordinate.iter().enumerate() {
                start[axis] = start[axis].min(component);
                end[axis] = end[axis].max(component);
            }
        }

        BoundingBox { start, end }
    }
}

impl ToString for Bitfield {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for y in 0..FIELD_SIZE {
            result.push('\n');
            result.push_str(&format!("Bitfield layer {}:", y));
            for z in (0..FIELD_SIZE).rev() {
                result.push('\n');
                for x in 0..FIELD_SIZE {
                    if self.is_filled(x, y, z) {
                        result.push_str(&"  ".on_white().to_string());
                    } else {
                        result.push_str(&"  ".on_truecolor(0, 0, 0).to_string());
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Bitfield;
    use colored::Colorize;
    use orientation::{Axis, Direction, Polarity};
    use tetrominos::Tetromino;

    fn bitboard_to_string(bb: u64) -> String {
        let mut result = String::new();
        for z in (0..8).rev() {
            result.push_str("\n ");
            for x in 0..8 {
                let index = x + z * 8;
                if (bb >> index) & 1 == 1 {
                    result.push_str(&"  ".on_white().to_string())
                } else {
                    result.push_str(&"  ".on_truecolor(0, 0, 0).to_string());
                }
            }
        }
        result
    }

    #[test]
    fn canonical() {
        let bitfield = Bitfield::canonical(Tetromino::T);

        println!("{}", bitfield.to_string());

        let bitboards = bitfield.compute_bitboards();

        for i in 0..bitboards.len() {
            println!("{}", bitboard_to_string(bitboards[i]));
        }
    }

    #[test]
    fn rotation() {
        let old_bitfield = Bitfield {
            value:
                0b0000_0000_0000_0000__0000_0000_0000_0000__0000_0100_0111_0000__0000_0000_0000_0000,
        };

        let old_bounding_box = old_bitfield.compute_bounding_box();

        println!("Old bitfield: {}", old_bitfield.to_string());
        println!("Old bounding box: {}", old_bounding_box.to_string());

        let new_bitfield = old_bitfield.rotate(
            Direction {
                axis: Axis::X,
                polarity: Polarity::Negative,
            },
            3,
        );

        let new_bounding_box = new_bitfield.compute_bounding_box();

        println!("New bitfield: {}", new_bitfield.to_string());
        println!("New bounding box: {}", new_bounding_box.to_string())
    }
}
