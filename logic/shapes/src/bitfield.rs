use colored::{Color, Colorize};
use orientation::{Axis, Direction};
use strum::EnumCount;

pub struct Bitfield {
    pub value: u64,
    pub dimensions: [usize; Axis::COUNT],
}

const FIELD_SIZE: usize = 4;
impl Bitfield {
    pub fn to_layers(&self) -> [u64; FIELD_SIZE] {
        todo!()
    }

    #[inline]
    pub fn index(x: usize, y: usize, z: usize) -> usize {
        x + y * FIELD_SIZE * FIELD_SIZE + z * FIELD_SIZE
    }

    #[inline]
    pub fn is_filled(&self, x: usize, y: usize, z: usize) -> bool {
        (self.value >> Self::index(x, y, z)) & 1 == 1
    }

    pub fn rotate(&self, along: Direction) -> Self {
        let mut output_value = 0;

        let [size_x, size_y, size_z] = self.dimensions;

        for z in 0..size_z {
            for y in 0..size_y {
                for x in 0..size_x {
                    let bit_value = (self.value >> Self::index(x, y, z)) & 1;
                    let [i, j, k] = along.rotate_coordinate([x, y, z], self.dimensions);
                    output_value |= bit_value << Self::index(i, j, k);
                }
            }
        }

        Self {
            value: output_value,
            dimensions: along.axis.rotate_dimensions(self.dimensions),
        }
    }
}

impl ToString for Bitfield {
    fn to_string(&self) -> String {
        let [size_x, size_y, size_z] = self.dimensions;
        let mut result = String::new();

        for y in 0..size_y {
            result.push('\n');
            result.push_str(&format!("Layer {}:", y));
            for z in 0..size_z {
                result.push('\n');
                for x in 0..size_x {
                    if self.is_filled(x, y, z) {
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
    use orientation::{Axis, Direction, Polarity};

    use super::Bitfield;

    #[test]
    pub fn it_works() {
        let bitfield = Bitfield {
            value:
                0b0000_0000_0000_0000__0000_0000_0000_0000__0000_0100_0111_0000__0000_0000_0000_0000,
            dimensions: [3, 3, 3],
        };

        println!("{}", bitfield.to_string());

        let new_bitfield = bitfield.rotate(Direction {
            axis: Axis::X,
            polarity: Polarity::Negative,
        });

        println!("{}", new_bitfield.to_string());
        println!("test");

        // output: 0b0000_0000_0000_0000__0000_0000_0100_0000__0000_0000_0111_0000__0000_0000_0000_0000

        // let binary_string = format!("{:064b}", new_bitfield.value);

        // let formatted_binary: String = binary_string
        //     .chars()
        //     .collect::<Vec<_>>()
        //     .chunks(4)
        //     .enumerate()
        //     .map(|(i, chunk)| {
        //         let chunk_str: String = chunk.iter().collect();
        //         if i > 0 && i % 4 == 0 {
        //             format!("_{}", chunk_str) // Add extra underscore every 16 characters
        //         } else {
        //             chunk_str
        //         }
        //     })
        //     .collect::<Vec<_>>()
        //     .join("_");

        // println!("{}", formatted_binary);
    }
}
