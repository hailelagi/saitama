use crate::board::{marker::Marker, Board};
use crate::decision::Decision;
use rand::seq::SliceRandom;

pub struct Simple;

impl Decision for Simple {
    fn choose_position(free_positions: &mut Vec<usize>, board: &Board) -> Option<usize> {
        for (i, marker) in board.state.iter().enumerate() {
            match (*marker, Marker::Empty) {
                (Marker::Empty, Marker::Empty) => free_positions.push(i),
                _ => (),
            };
        }

        free_positions.choose(&mut rand::thread_rng()).copied()
    }
}

#[cfg(test)]
mod test {
    use crate::board::{
        marker::Marker,
        settings::{self, Difficulty::Easy},
        Board,
    };

    use super::Simple;

    use crate::decision::Decision;

    #[test]
    fn test_selects_a_position_initial_input() {
        let mut init = [Marker::Empty; 9];
        let player_marker = Marker::X;
        init[0] = player_marker;

        let settings = settings::Settings::build(Easy, player_marker);

        let board = Board {
            state: init,
            markers_placed: 1,
            settings,
        };

        let result = Simple::choose_position(&mut Vec::new(), &board);

        if let Some(_) = result {
            ();
        } else {
            panic!("did not find min max position")
        }
    }
}
