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

#[cfg(test)]
mod tests {

    use orientation::Polarity;
    use queue::Parsing;

    use super::*;

    #[test]
    pub fn it_works() {
        let mut game_state = GameState::<6, 8, 6>::from(Queue::parse("[I]p1").unwrap());

        let start = std::time::Instant::now();
        // println!("{:?}", game_state.piece.shape_id.orientation);
        // game_state.perform(Action::Rotate {
        //     direction: Direction::from((Polarity::Positive, Axis::Y)),
        // });
        // println!("{:?}", game_state.piece.shape_id.orientation);
        game_state.perform(Action::HardDrop);
        let duration = start.elapsed();

        println!("Performing action took: {:?}", duration);
        println!("{}", game_state.to_string());
    }
}

#[derive(Debug)]
pub struct GameState<const W: usize, const H: usize, const D: usize> {
    queue: Queue,
    piece: Piece,
    playfield: [Layer<W, D>; H],
}

impl<const W: usize, const H: usize, const D: usize> GameState<W, H, D> {
    pub fn perform(&mut self, action: Action) -> bool {
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
                if fits {
                    self.piece.translate(offset)
                }
                fits
            }
            Action::SoftDrop => {
                let offset = [0, -1, 0];
                let fits = self.fits(offset, None);
                if fits {
                    self.piece.translate(offset);
                }
                fits
            }
            Action::HardDrop => {
                let offset = [0, -1, 0];
                while self.fits(offset, None) {
                    self.piece.translate(offset);
                }
                self.place_piece();
                self.new_piece();
                self.clear_lines();
                true
            }
            Action::Rotate { direction } => {
                // let fits = self.fits([0, 0, 0], Some(direction));
                self.piece.rotate(direction);
                // fits
                true
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

    fn clear_lines(&mut self) {
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
    }

    fn place_piece(&mut self) {
        let [x, y, z] = self.piece.position;

        let Shape {
            bitboards,
            bounding_box,
        } = self.piece.shape();
        let [_, start_y, _] = bounding_box.start;
        let [_, end_y, _] = bounding_box.end;

        println!("placing {:?} at {x},{y},{z}", self.piece.shape_id.variant); // TODO

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

/*

impl<const W: usize, const H: usize, const D: usize> Display for GameState<W, H, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for (l, &layer) in self.playfield.iter().enumerate().rev() {
            output.push_str(&format!("Layer {l}:\n"));
            output.push_str(&layer.to_string());
            output.push_str("\n");
        }

        f.write_str(&output)
    }
}


*/
impl<const W: usize, const H: usize, const D: usize> From<Queue> for GameState<W, H, D> {
    fn from(mut queue: Queue) -> Self {
        let variant = queue.next();

        let piece = Piece::from::<W, H, D>(variant);
        let playfield = [Default::default(); H];

        Self {
            queue,
            piece,
            playfield,
        }
    }
}

impl<const W: usize, const H: usize, const D: usize> Default for GameState<W, H, D> {
    fn default() -> Self {
        Self::from(Queue::default())
    }
}
