use std::fmt::Display;

use layer::Layer;
use orientation::{Axis, Rotation};
use piece::Piece;
use queue::Queue;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn it_works() {
        let mut game_state = GameState::<8, 20, 8>::default();

        // game_state.perform(Action::SoftDrop);

        println!("{:?}", game_state.current_piece.position);
    }
}

pub enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    SoftDrop,
    HardDrop,
    Rotate { axis: Axis, rotation: Rotation },
}

#[derive(Debug)]
pub struct GameState<const W: usize, const H: usize, const D: usize> {
    queue: Queue,
    current_piece: Piece,
    playfield: [Layer<W, D>; H],
}

impl<const W: usize, const H: usize, const D: usize> GameState<W, H, D> {
    pub fn perform(&mut self, action: Action) {
        match action {
            movement @ (Action::MoveForward
            | Action::MoveBackward
            | Action::MoveLeft
            | Action::MoveRight) => {
                let offset = match movement {
                    Action::MoveForward => [0, 0, 1],
                    Action::MoveBackward => [0, 0, -1],
                    Action::MoveLeft => [-1, 0, 0],
                    Action::MoveRight => [1, 0, 0],
                    _ => unreachable!(),
                };
            }
            Action::SoftDrop => todo!(),
            Action::HardDrop => todo!(),
            Action::Rotate { axis, rotation } => todo!(),
        }
    }

    fn fits() -> bool {
        todo!()
    }
}

impl<const W: usize, const H: usize, const D: usize> Display for GameState<W, H, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        f.write_str(&output)
    }
}

impl<const W: usize, const H: usize, const D: usize> Default for GameState<W, H, D> {
    fn default() -> Self {
        let mut queue = Queue::default();
        let variant = queue.next();

        let current_piece = Piece::from::<W, H, D>(variant);
        let playfield = [Default::default(); H];

        Self {
            queue,
            current_piece,
            playfield,
        }
    }
}
