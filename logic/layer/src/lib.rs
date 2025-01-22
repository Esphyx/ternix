use colored::{Color, Colorize};
use strum::{EnumCount, IntoEnumIterator};
use tetrominos::Tetromino;

// TODO change the layer struct to contain a bitboard for every tetromino variant
#[derive(Debug, Default, Clone, Copy)]
pub struct Layer<const W: usize, const D: usize> {
    pub bitboards: [u64; Tetromino::COUNT],
}

impl<const W: usize, const D: usize> Layer<W, D> {
    pub fn is_full(&self) -> bool {
        self.occupied() == !0
    }

    pub fn occupied(&self) -> u64 {
        let mut combined = 0;
        for i in 0..Tetromino::COUNT {
            combined |= self.bitboards[i];
        }
        combined
    }
}

impl<const W: usize, const D: usize> ToString for Layer<W, D> {
    fn to_string(&self) -> String {
        let mut output = String::from(" ");

        for d in 0..D {
            for x in 0..W {
                let i = x + d * 8;

                let mut is_free = true;
                for (variant, &bitboard) in Tetromino::iter().zip(self.bitboards.iter()) {
                    if (bitboard >> i) & 1 == 1 {
                        output.push_str(&"  ".on_color(variant.to_color()).to_string());
                        is_free = false;
                    }
                }

                if is_free {
                    output.push_str(
                        &"  "
                            .on_color(Color::TrueColor { r: 0, g: 0, b: 0 })
                            .to_string(),
                    );
                }
            }
            output.push_str("\n ");
        }

        output
    }
}
