use colored::Color;
use strum_macros::{EnumCount, EnumIter, EnumString};

#[derive(Debug, EnumCount, EnumIter, EnumString, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(usize)]
pub enum Tetromino {
    I,
    O,
    T,
    L,
    S,
    B,
    D,
    F,
}

impl Tetromino {
    pub fn to_color(&self) -> Color {
        match self {
            Tetromino::I => Color::TrueColor {
                r: 0,
                g: 255,
                b: 255,
            },
            Tetromino::O => Color::TrueColor {
                r: 255,
                g: 255,
                b: 0,
            },
            Tetromino::T => Color::TrueColor {
                r: 127,
                g: 0,
                b: 127,
            },
            Tetromino::L => Color::TrueColor {
                r: 255,
                g: 127,
                b: 0,
            },
            Tetromino::S => Color::TrueColor { r: 0, g: 255, b: 0 },
            Tetromino::B => Color::TrueColor {
                r: 127,
                g: 47,
                b: 0,
            },
            Tetromino::D => Color::TrueColor {
                r: 63,
                g: 63,
                b: 63,
            },
            Tetromino::F => Color::TrueColor {
                r: 191,
                g: 191,
                b: 191,
            },
        }
    }
}

impl From<usize> for Tetromino {
    fn from(value: usize) -> Self {
        match value {
            0 => Tetromino::I,
            1 => Tetromino::O,
            2 => Tetromino::T,
            3 => Tetromino::L,
            4 => Tetromino::S,
            5 => Tetromino::B,
            6 => Tetromino::D,
            7 => Tetromino::F,
            _ => unreachable!(),
        }
    }
}
