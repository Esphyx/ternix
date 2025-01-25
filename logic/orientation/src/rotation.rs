use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, Default, EnumCount, EnumIter)]
pub enum Rotation {
    #[default]
    Identity,
    Generator,
    Double,
    Prime,
}

impl Rotation {
    pub fn next(&self) -> Self {
        match self {
            Rotation::Identity => Rotation::Generator,
            Rotation::Generator => Rotation::Double,
            Rotation::Double => Rotation::Prime,
            Rotation::Prime => Rotation::Identity,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Rotation::Identity => Rotation::Prime,
            Rotation::Generator => Rotation::Identity,
            Rotation::Double => Rotation::Generator,
            Rotation::Prime => Rotation::Double,
        }
    }
}