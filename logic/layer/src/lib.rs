use colored::Colorize;
use strum::{EnumCount, IntoEnumIterator};
use tetrominos::Tetromino;

pub const MAX_SIZE: usize = 8;
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Layer<const W: usize, const D: usize> {
    pub bitboards: [u64; Tetromino::COUNT],
}

impl<const W: usize, const D: usize> Layer<W, D> {
    const fn mask() -> u64 {
        let mut mask = 0;
        let mut i = 0;
        while i < D {
            mask |= ((1 << W) - 1) << (i * MAX_SIZE);
            i += 1;
        }
        mask
    }

    pub fn is_full(&self) -> bool {
        self.occupied() & Self::mask() == Self::mask()
    }

    pub fn occupied(&self) -> u64 {
        let mut combined = 0;
        for i in 0..Tetromino::COUNT {
            combined |= self.bitboards[i];
        }
        combined
    }

    pub fn clear(&mut self) {
        self.bitboards = [0; Tetromino::COUNT]
    }
}

impl<const W: usize, const D: usize> ToString for Layer<W, D> {
    fn to_string(&self) -> String {
        let mut output = String::from(" ");

        for z in (0..D).rev() {
            for x in 0..W {
                let i = x + z * MAX_SIZE;

                let mut is_free = true;
                for (variant, &bitboard) in Tetromino::iter().zip(self.bitboards.iter()) {
                    if (bitboard >> i) & 1 == 1 {
                        output.push_str(&"  ".on_color(variant.to_color()).to_string());
                        is_free = false;
                        break;
                    }
                }

                if is_free {
                    output.push_str(&"  ".on_truecolor(0, 0, 0).to_string());
                }
            }
            output.push_str("\n ");
        }

        output
    }
}
