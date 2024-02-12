use crate::board::{marker::Marker, Board};
use crate::decision::Decision;

use super::simple::Simple;

pub struct CombinatorialSearch;

const WINNING_POSITIONS: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

const PERMUTATIONS: [[usize; 2]; 3] = [[0, 1], [0, 2], [1, 2]];

impl Decision for CombinatorialSearch {
    fn choose_position(free_positions: &mut Vec<usize>, board: &Board) -> Option<usize> {
        let state = board.state;

        for (i, marker) in state.iter().enumerate() {
            match (*marker, Marker::Empty) {
                (Marker::Empty, Marker::Empty) => free_positions.push(i),
                _ => (),
            };
        }

        let favorable_positions = enumerate_positions(board, board.settings.opponent_marker);
        if favorable_positions.len() != 0 {
            return Some(favorable_positions[0]);
        }

        let must_defend = enumerate_positions(board, board.settings.player_marker);
        if must_defend.len() != 0 {
            return Some(must_defend[0]);
        } else {
            Simple::choose_position(free_positions, board)
        }
    }
}

fn enumerate_positions(board: &Board, marker: Marker) -> Vec<usize> {
    let mut risky_positions = Vec::new();

    for winning_position in WINNING_POSITIONS {
        for permutation in PERMUTATIONS {
            if board.state[winning_position[permutation[0]]] == marker
                && board.state[winning_position[permutation[1]]] == marker
            {
                let missing = 3 - (permutation[0] + permutation[1]);
                if let Marker::Empty = board.state[winning_position[missing]] {
                    risky_positions.push(winning_position[missing]);
                    continue;
                }
            }
        }
    }
    risky_positions
}

#[cfg(test)]
mod test {
    use super::CombinatorialSearch;

    use crate::board::settings;
    use crate::board::{marker::Marker, settings::Difficulty::HardEnum, Board};
    use crate::decision::Decision;

    #[test]
    fn test_selects_random_valid_position() {
        let mut init = [Marker::Empty; 9];
        init[0] = Marker::X;

        let player_marker = Marker::X;
        init[0] = player_marker;

        let settings = settings::Settings::build(HardEnum, player_marker);

        let board = Board {
            state: init,
            markers_placed: 1,
            settings,
        };

        let result = CombinatorialSearch::choose_position(&mut Vec::new(), &board);

        if let Some(_) = result {
            ();
        } else {
            panic!("did not find a random position to place")
        }
    }
}
