use orientation::Direction;
use strum_macros::EnumIter;

#[derive(EnumIter)]
pub enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    SoftDrop,
    HardDrop,
    Rotate { direction: Direction },
}
