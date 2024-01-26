// AI that chooses a position on the board
// According to the difficulty EASY or HARD
// easy - a simple ai that chooses a random valid position on the board
// hard - a min-max algorithm that chooses a valid position + alpha-beta pruning.

// step 1: construct a naive search tree

// https://en.wikipedia.org/wiki/Minimax#pseudocode
//
// function minimax(node, depth, maximizingPlayer) is
//     if depth = 0 or node is a terminal node then
//         return the heuristic value of node
//     if maximizingPlayer then
//         value := −∞
//         for each child of node do
//             value := max(value, minimax(child, depth − 1, FALSE))
//         return value
//     else (* minimizing player *)
//         value := +∞
//         for each child of node do
//             value := min(value, minimax(child, depth − 1, TRUE))
//         return value

use crate::board::Board;
use crate::decision::Decision;

pub struct Minimax;

impl Decision for Minimax {
    fn choose_position(_: &mut Vec<usize>, _board: &Board) -> Option<usize> {
        todo!("minmax described above")
    }
}
