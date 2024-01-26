use crate::board::{marker::Marker, Board};
use crate::decision::Decision;
pub struct CombinatorialSearch;

impl Decision for CombinatorialSearch {
    fn choose_position(free_positions: &mut Vec<usize>, board: &Board) -> Option<usize> {
        let state = board.state;

        for (i, marker) in state.iter().enumerate() {
            match (*marker, Marker::Empty) {
                (Marker::Empty, Marker::Empty) => free_positions.push(i),
                _ => (),
            };
        }

        let favorable_positions = favorable_positions(board);
        if favorable_positions.len() != 0 {
            return Some(favorable_positions[0]);
        }

        let must_defend = vulnerable_positions(board);
        if must_defend.len() != 0 {
            return Some(must_defend[0]);
        } else {
            None
        }
    }
}

fn vulnerable_positions(board: &Board) -> Vec<usize> {
    let mut risky_positions = Vec::new();
    let (_, player_marker) = board.get_markers();
    let winning_positions = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    let permutations = [[0usize, 1], [0, 2], [1, 2]];

    for winning_position in winning_positions {
        for permutation in permutations {
            if board.state[winning_position[permutation[0]]] == player_marker
                && board.state[winning_position[permutation[1]]] == player_marker
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

fn favorable_positions(board: &Board) -> Vec<usize> {
    let mut good_positions = Vec::new();
    let (opponent_marker, _) = board.get_markers();
    let winning_positions = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    let permutations = [[0usize, 1], [0, 2], [1, 2]];

    for winning_position in winning_positions {
        for permutation in permutations {
            if board.state[winning_position[permutation[0]]] == opponent_marker
                && board.state[winning_position[permutation[1]]] == opponent_marker
            {
                let missing = 3 - (permutation[0] + permutation[1]);
                if let Marker::Empty = board.state[winning_position[missing]] {
                    good_positions.push(winning_position[missing]);
                }
            }
        }
    }
    good_positions
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

        let settings = &settings::Settings::build(HardEnum, player_marker);

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
