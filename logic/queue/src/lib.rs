use rand::Rng;
use std::collections::{HashMap, VecDeque};
use strum::{EnumCount, IntoEnumIterator};
use tetrominos::Tetromino;

#[derive(Debug, Default)]
pub struct Queue {
    sequence: VecDeque<Pattern>,
    hold: Option<Tetromino>,
    can_swap: bool,
}

#[derive(Debug)]
struct Pattern {
    tetromino_set: [usize; Tetromino::COUNT],
    draw_count: usize,
}

impl Queue {
    pub fn next(&mut self) -> Tetromino {
        let mut pattern = self.sequence.pop_front().unwrap_or_default();
        if let Some(variant) = pattern.draw() {
            self.sequence.push_front(pattern);
            return variant;
        }
        self.next()
    }
}

impl Pattern {
    pub fn size(&self) -> usize {
        self.tetromino_set.iter().sum()
    }

    pub fn draw(&mut self) -> Option<Tetromino> {
        let total_weight = self.size();
        if total_weight == 0 {
            return None;
        }

        let bar = rand::thread_rng().gen_range(0..total_weight);

        let mut cumulative_weight = 0;
        for (i, weight) in self.tetromino_set.iter_mut().enumerate() {
            cumulative_weight += *weight;
            if bar <= cumulative_weight {
                *weight -= 1;
                self.draw_count -= 1;
                return Some(Tetromino::from(i));
            }
        }

        None
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            tetromino_set: [1; Tetromino::COUNT],
            draw_count: Tetromino::COUNT,
        }
    }
}
