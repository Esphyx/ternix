use std::hash::Hash;

use action::Action;
use colored::Colorize;
use layer::Layer;
use orientation::{Axis, Direction};
use piece::Piece;
use queue::Queue;
use shapes::{BoundingBox, Shape};
use strum::{EnumCount, IntoEnumIterator};
use tetrominos::Tetromino;

mod action;
mod brute_forcing;
mod heuristic;

#[cfg(test)]
mod tests {

    use queue::Parsing;
    use std::{collections::HashSet, time::Instant};

    use super::*;

    #[test]
    pub fn heuristic_testing() {
        let mut game_state = GameState::<4, 8, 4>::from(Queue::parse("[IIII]p4").unwrap());
        game_state.perform(Action::HardDrop, true);

        game_state.perform(Action::HardDrop, true);
        println!("{}", game_state.to_string());
        println!("{}", game_state.heuristic());
    }

    #[test]
    pub fn it_works() {
        let start = Instant::now();
        let game_state = GameState::<4, 8, 4>::default();

        let mut map = HashSet::new();
        let mut hashset_size = 0;
        brute_forcing::algorithm(game_state, 200, &mut map, &mut hashset_size);

        println!("{}", map.iter().collect::<Vec<_>>().len());

        println!("Program took: {:?}", start.elapsed());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState<const W: usize, const H: usize, const D: usize> {
    history: Vec<Action>,
    queue: Queue,
    piece: Piece,
    playfield: [Layer<W, D>; H],
}

impl<const W: usize, const H: usize, const D: usize> GameState<W, H, D> {
    pub fn performable_actions(&mut self) -> Vec<Action> {
        // TODO: rotating more than twice is not necessary
        // TODO: just first try without softdropping

        Action::get_all_actions()
            .into_iter()
            .filter(|&action| {
                let last = 3;
                let is_fine = if self.history.len() > last {
                    self.history[self.history.len() - last..]
                        .iter()
                        .any(|&element| element == Action::HardDrop)
                } else {
                    true
                };

                self.perform(action, false) && is_fine
            })
            .collect()
    }

    pub fn perform(&mut self, action: Action, should_perform: bool) -> bool {
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

                let fits = self.fits(offset, None);
                if fits && should_perform {
                    self.history.push(action);
                    self.piece.translate(offset)
                }
                fits
            }
            Action::SoftDrop => {
                let offset = [0, -1, 0];
                let fits = self.fits(offset, None);
                if fits && should_perform {
                    self.history.push(action);
                    self.piece.translate(offset);
                }
                fits
            }
            Action::HardDrop => {
                if should_perform {
                    let offset = [0, -1, 0];
                    while self.fits(offset, None) {
                        self.piece.translate(offset);
                    }
                    self.place_piece();
                    self.new_piece();
                    self.clear_lines();
                    self.history.push(action);
                }
                true
            }
            Action::Rotate { direction } => {
                let fits = self.fits([0, 0, 0], Some(direction));
                if fits && should_perform {
                    self.piece.rotate(direction);
                    self.history.push(action);
                }
                fits
            }
        }
    }

    fn new_piece(&mut self) {
        self.piece = Piece::from::<W, H, D>(self.queue.next())
    }

    fn translate(bitboard: u64, x: isize, z: isize) -> u64 {
        let shift_value = x + z * layer::MAX_SIZE as isize;

        if shift_value > 0 {
            bitboard << shift_value
        } else {
            bitboard >> -shift_value
        }
    }

    fn clear_lines(&mut self) -> usize {
        let mut l = 0;
        let mut cleared_layer_count = 0;
        while l < H - cleared_layer_count {
            let layer = &mut self.playfield[l];
            if layer.is_full() {
                cleared_layer_count += 1;
                for i in l..H - cleared_layer_count {
                    self.playfield[i] = self.playfield[i + 1];
                }
            }
            l += 1;
        }

        for i in 0..cleared_layer_count {
            self.playfield[H - i - 1].clear();
        }

        if cleared_layer_count >= 1 {
            println!("{}", self.to_string());
        }

        cleared_layer_count
    }

    fn place_piece(&mut self) {
        let [x, y, z] = self.piece.position;

        let Shape {
            bitboards,
            bounding_box,
            ..
        } = self.piece.shape();
        let [_, start_y, _] = bounding_box.start;
        let [_, end_y, _] = bounding_box.end;

        for i in start_y..=end_y {
            self.playfield[(y + i as isize) as usize].bitboards
                [self.piece.shape_id.variant as usize] |= Self::translate(bitboards[i], x, z);
        }
    }

    fn out_of_bounds(position: [isize; Axis::COUNT]) -> bool {
        let [x, y, z] = position;

        (x < 0 || y < 0 || z < 0) || (x >= W as isize || y >= H as isize || z >= D as isize)
    }

    fn fits(&self, offset: [isize; Axis::COUNT], along: Option<Direction>) -> bool {
        let Piece {
            mut shape_id,
            position,
        } = self.piece;
        if let Some(direction) = along {
            shape_id = shape_id.rotate(direction);
        }

        let [x, y, z] = position;
        let [dx, dy, dz] = offset;
        let [x, y, z] = [x + dx, y + dy, z + dz];

        let shape: &Shape = (&shape_id).into();
        let &Shape {
            bitboards,
            bounding_box,
            ..
        } = shape;

        let BoundingBox { start, end } = bounding_box;

        let [start_x, start_y, start_z] = start;
        let [end_x, end_y, end_z] = end;

        if Self::out_of_bounds([
            x + start_x as isize,
            y + start_y as isize,
            z + start_z as isize,
        ]) || Self::out_of_bounds([x + end_x as isize, y + end_y as isize, z + end_z as isize])
        {
            return false;
        }

        for i in start_y..=end_y {
            let layer = &self.playfield[(y + i as isize) as usize];

            if layer.occupied() & Self::translate(bitboards[i], x, z) != 0 {
                return false;
            }
        }

        true
    }
}

impl<const W: usize, const H: usize, const D: usize> ToString for GameState<W, H, D> {
    fn to_string(&self) -> String {
        let mut result = String::new();

        for z in (0..D).rev() {
            for y in 0..H {
                let layer = self.playfield[y];
                for x in 0..W {
                    let index = x + z * layer::MAX_SIZE;

                    let mut is_free = true;
                    for (variant, &bitboard) in Tetromino::iter().zip(layer.bitboards.iter()) {
                        if (bitboard >> index) & 1 == 1 {
                            result.push_str(&"  ".on_color(variant.to_color()).to_string());
                            is_free = false;
                            break;
                        }
                    }

                    if is_free {
                        result.push_str(&"  ".on_truecolor(0, 0, 0).to_string());
                    }
                }
                result.push_str("  ");
            }
            result.push_str("\n");
        }

        result
    }
}

impl<const W: usize, const H: usize, const D: usize> From<Queue> for GameState<W, H, D> {
    fn from(mut queue: Queue) -> Self {
        let variant = queue.next();

        let piece = Piece::from::<W, H, D>(variant);
        let playfield = [Default::default(); H];

        Self {
            history: Vec::new(),
            queue,
            piece,
            playfield,
        }
    }
}

impl<const W: usize, const H: usize, const D: usize> Hash for GameState<W, H, D> {
    fn hash<T: std::hash::Hasher>(&self, state: &mut T) {
        self.queue.hash(state);
        self.piece.hash(state);
        self.playfield.hash(state);
    }
}

impl<const W: usize, const H: usize, const D: usize> Default for GameState<W, H, D> {
    fn default() -> Self {
        Self::from(Queue::default())
    }
}
