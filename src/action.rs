use orientation::{Axis, Direction, Polarity};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    SoftDrop,
    HardDrop,
    Rotate { direction: Direction },
}

impl Action {
    pub fn is_movement(&self) -> bool {
        matches!(
            self,
            Self::MoveForward | Self::MoveBackward | Self::MoveLeft | Self::MoveRight
        )
    }

    pub fn get_all_actions() -> Vec<Self> {
        let mut actions = vec![
            Self::MoveForward,
            Self::MoveBackward,
            Self::MoveLeft,
            Self::MoveRight,
            Self::SoftDrop,
            Self::HardDrop,
        ];

        for axis in Axis::iter() {
            for polarity in Polarity::iter() {
                let direction = Direction::from((polarity, axis));
                actions.push(Self::Rotate { direction })
            }
        }

        actions
    }
}
