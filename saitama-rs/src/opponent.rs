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

use crate::board::{board::Board, marker::Marker};
use rand::seq::SliceRandom;

pub trait Decision {
    fn choose_position(board: &Board) -> Option<usize>;
}

pub fn min_max(board: Board) -> Board {
    let state = board.state;
    let mut free_positions = Vec::new();

    for (i, marker) in state.iter().enumerate() {
        match (*marker, Marker::Empty) {
            (Marker::Empty, Marker::Empty) => free_positions.push(i),
            _ => (),
        };
    }

    // search tree

    return board;
}

pub struct SimpleAI;

impl Decision for SimpleAI {
    fn choose_position(board: &Board) -> Option<usize> {
        let mut free_positions = Vec::new();

        for (i, marker) in board.state.iter().enumerate() {
            match (*marker, Marker::Empty) {
                (Marker::Empty, Marker::Empty) => free_positions.push(i),
                _ => (),
            };
        }

        return free_positions.choose(&mut rand::thread_rng()).copied();
    }
}

#[cfg(test)]
mod test {
    use crate::board::board::Board;
    use crate::board::marker::Marker;
    use crate::opponent::{min_max, Decision, SimpleAI};

    #[test]
    fn test_selects_a_position_initial_input() {
        let mut init = [Marker::Empty; 9];
        init[0] = Marker::X;

        let board = Board {
            state: init,
            markers_placed: 1,
        };

        let result = min_max(board);

        let found_ai_mark = result.state.iter().find(|pos| **pos == Marker::O);

        if let Some(_) = found_ai_mark {
            ();
        } else {
            panic!("did not find min max position")
        }
    }

    #[test]
    fn test_selects_random_valid_position() {
        let mut init = [Marker::Empty; 9];
        init[0] = Marker::X;

        let board = Board {
            state: init,
            markers_placed: 1,
        };

        let result = SimpleAI::choose_position(&board);

        if let Some(_) = result {
            ();
        } else {
            panic!("did not find a random position to place")
        }
    }
}
