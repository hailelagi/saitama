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

pub struct SimpleAI;
pub struct Minimax;

pub struct CombinatorialSearch;


impl Decision for CombinatorialSearch{
    fn choose_position(board: &Board) -> Option<usize> {
        let state = board.state;
        let mut free_positions = Vec::new();

        for (i, marker) in state.iter().enumerate() {
            match (*marker, Marker::Empty) {
                (Marker::Empty, Marker::Empty) => free_positions.push(i),
                _ => (),
            };
        }

        let favorable_positions = Self::favorable_positions(board);
        if favorable_positions.len() != 0{
            return Some(favorable_positions[0]);
        }

        let must_defend = Self::vulnerable_positions(board);
        if must_defend.len() != 0{
            return Some(must_defend[0]);
        }

        return SimpleAI::choose_position(board);
    }
}

impl CombinatorialSearch{
    fn vulnerable_positions(board : &Board) -> Vec<usize>{
        let mut risky_positions = Vec::new();
        let (_, player_marker) = board.get_markers();
        let winning_positions = [[0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 3, 6], [1, 4, 7], [2, 5, 8], [0, 4, 8], [2, 4, 6]];
        let permutations = [[0usize, 1], [0, 2], [1, 2]];

        for winning_position in winning_positions{
            for permutation in permutations{
                if board.state[winning_position[permutation[0]]] == player_marker && board.state[winning_position[permutation[1]]] == player_marker{
                    let missing = 3 - (permutation[0] + permutation[1]);
                    if let Marker::Empty = board.state[winning_position[missing]]{
                        risky_positions.push(winning_position[missing]);
                        continue;
                    }                   
                }
            }
        }
        risky_positions
    }

    fn favorable_positions(board : &Board) -> Vec<usize>{
        let mut good_positions = Vec::new();
        let (opponent_marker, _) = board.get_markers();
        let winning_positions = [[0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 3, 6], [1, 4, 7], [2, 5, 8], [0, 4, 8], [2, 4, 6]];
        let permutations = [[0usize, 1], [0, 2], [1, 2]];

        for winning_position in winning_positions{
            for permutation in permutations{                
                if board.state[winning_position[permutation[0]]] == opponent_marker && board.state[winning_position[permutation[1]]] == opponent_marker{
                    let missing = 3 - (permutation[0] + permutation[1]);
                    if let Marker::Empty = board.state[winning_position[missing]]{
                        good_positions.push(winning_position[missing]);
                    }
                }
            }
        } 
        good_positions
    }



}

impl Decision for Minimax {
    fn choose_position(board: &Board) -> Option<usize> {
        let state = board.state;
        let mut free_positions = Vec::new();

        for (i, marker) in state.iter().enumerate() {
            match (*marker, Marker::Empty) {
                (Marker::Empty, Marker::Empty) => free_positions.push(i),
                _ => (),
            };
        }

        // todo: search tree
        return None;
    }
}

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
    use crate::opponent::{Decision, Minimax, SimpleAI};
    use crate::world::World;

    #[test]
    fn test_selects_a_position_initial_input() {
        let mut init = [Marker::Empty; 9];
        init[0] = Marker::X;

        let board = Board {
            state: init,
            markers_placed: 1,
            world: &World::build(crate::world::Difficulty::Easy, Marker::O)
        };

        let result = Minimax::choose_position(&board);

        if let Some(_) = result {
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
            world: &World::build(crate::world::Difficulty::Easy, Marker::O)
        };

        let result = SimpleAI::choose_position(&board);

        if let Some(_) = result {
            ();
        } else {
            panic!("did not find a random position to place")
        }
    }
}
