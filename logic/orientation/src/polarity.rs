use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, Default, EnumCount, EnumIter, PartialEq, Eq)]
pub enum Polarity {
    #[default]
    Positive,
    Negative,
}

impl Polarity {
    pub fn cross(&self, other: Self) -> Self {
        // isomorphic to XOR for a homogeneous handed system
        // XNOR for a heterogeneous handed system
        match (self, other) {
            (Polarity::Positive, Polarity::Positive) => Polarity::Positive,
            (Polarity::Positive, Polarity::Negative) => Polarity::Negative,
            (Polarity::Negative, Polarity::Positive) => Polarity::Negative,
            (Polarity::Negative, Polarity::Negative) => Polarity::Positive,
        }
    }
}
