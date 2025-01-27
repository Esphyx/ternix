use crate::{action::Action, GameState};

impl<const W: usize, const H: usize, const D: usize> GameState<W, H, D> {
    pub fn heuristic(&self) -> f64 {
        let mut total = self
            .history
            .iter()
            .filter(|&action| matches!(action, Action::HardDrop))
            .count() as f64;

        for i in shapes::MAX_SIZE..H {
            if self.playfield[i].occupied() != 0 {
                total = -1.0;
            }
        }

        total
    }
}
