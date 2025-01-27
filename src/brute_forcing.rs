use std::collections::HashSet;

use crate::GameState;

pub fn children<const W: usize, const H: usize, const D: usize>(
    mut game_state: GameState<W, H, D>,
) -> Vec<GameState<W, H, D>> {
    let mut children = Vec::new();
    let actions = game_state.performable_actions();

    for action in actions {
        let mut child = game_state.clone();
        child.perform(action, true);
        if child.heuristic() > 0.0 {
            children.push(child);
        }
    }

    children.sort_by(|a, b| a.heuristic().total_cmp(&b.heuristic()));
    children
}

pub fn algorithm<const W: usize, const H: usize, const D: usize>(
    game_state: GameState<W, H, D>,
    depth: usize,
    memoization: &mut HashSet<GameState<W, H, D>>,
    size: &mut usize,
) {
    if depth <= 0 || memoization.contains(&game_state) || game_state.heuristic() < 0.0 {
        return;
    }

    println!("explored {size} states");
    println!("{}", game_state.to_string());

    memoization.insert(game_state.clone());
    *size += 1;

    for child in children(game_state) {
        algorithm(child, depth - 1, memoization, size);
    }
}
