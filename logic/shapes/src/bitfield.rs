use colored::Colorize;
use orientation::Direction;

use crate::bounding_box::BoundingBox;

pub struct Bitfield {
    pub value: u64,
    pub bounding_box: BoundingBox,
}

const FIELD_SIZE: usize = 4;
impl Bitfield {
    #[inline]
    pub fn index(x: usize, y: usize, z: usize) -> usize {
        x + y * FIELD_SIZE * FIELD_SIZE + z * FIELD_SIZE
    }

    #[inline]
    pub fn is_filled(&self, x: usize, y: usize, z: usize) -> bool {
        (self.value >> Self::index(x, y, z)) & 1 == 1
    }

    pub fn rotate(&self, along: Direction) -> Self {
        let mut value = 0;
        let bounding_box = self.bounding_box.rotate(along);

        Self {
            value,
            bounding_box,
        }
    }

    pub fn to_layers(&self) -> [u64; FIELD_SIZE] {
        todo!()
        // unrelated
    }
}

impl ToString for Bitfield {
    fn to_string(&self) -> String {
        let [size_x, size_y, size_z] = self.bounding_box.size();
        let [start_x, start_y, start_z] = self.bounding_box.start;

        let mut result = String::new();
        for y in 0..size_y {
            result.push('\n');
            result.push_str(&format!("Layer {}:", y));
            for z in 0..size_z {
                result.push('\n');
                for x in 0..size_x {
                    if self.is_filled(x + start_x, y + start_y, z + start_z) {
                        result.push_str(&"  ".on_blue().to_string());
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
    use crate::bounding_box::BoundingBox;
    use orientation::{Axis, Direction, Polarity};

    #[test]
    pub fn it_works() {
        let bitfield = Bitfield {
            value:
                0b0000_0000_0000_0000__0000_0000_0000_0000__0000_0100_0111_0000__0000_0000_0000_0000,
            bounding_box: BoundingBox {
                start: [0, 0, 0],
                end: [3, 3, 3],
            },
        };

        println!("{}", bitfield.to_string());

        let new_bitfield = bitfield.rotate(Direction {
            axis: Axis::X,
            polarity: Polarity::Negative,
        });

        println!("{}", new_bitfield.to_string());
    }
}
