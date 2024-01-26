use crate::board::settings::Difficulty;
use crate::board::Board;

use self::combinatorial_search::CombinatorialSearch;
use self::mini_max::Minimax;
use self::simple::Simple;

mod combinatorial_search;
mod mini_max;
mod simple;

pub trait Decision {
    fn choose_position(free_positions: &mut Vec<usize>, board: &Board) -> Option<usize>;
}

pub fn choose_position(difficulty: Difficulty, board: &Board) -> Option<usize> {
    let mut free_positions = Vec::new();

    match difficulty {
        Difficulty::Easy => Simple::choose_position(&mut free_positions, board),
        Difficulty::HardSearchTree => Minimax::choose_position(&mut free_positions, board),
        Difficulty::HardEnum => CombinatorialSearch::choose_position(&mut free_positions, board),
    }
}
