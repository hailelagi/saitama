use crate::board::marker::Marker;
use crate::world::World;

pub enum Outcome {
    Win(Marker),
    Undecided,
    Draw,
}

// A sort of clever way to represent the board of a tic tac toe game is using a bitmap
// of unsigned ints The mapping of markers is roughly:
// X => 11
// 0 => 01
// Empty => 00
// You can then fit it nicely inside a tiny 16bit array `0000 0000 0000 0000`
// to determine an `Outcome` by comparing the bits to the win condition
// https://rclayton.silvrback.com/winner-in-tic-tac-toe
// However this is such an incredibly simple problem idk why anyone would take it so far?
// Apparently you can hash the winstates and do an O(1) lookup too?
// but you can like do that anyway with a hashmap or a prealloc'd array [u8;8]
pub struct Board {
    pub state: [Marker; 9],
    pub markers_placed: i32,
}

impl Board {
    pub fn new(_world: &World) -> Self {
        Board {
            state: [Marker::Empty; 9],
            markers_placed: 0,
        }
    }

    pub fn place_position(&mut self, position: usize, marker: Marker) -> Option<Marker> {
        let board_state = &mut self.state;

        match board_state[position] {
            Marker::Empty => {
                self.markers_placed += 1;
                board_state[position] = marker;
                Some(marker)
            }
            _ => None,
        }
    }

    pub fn validate_game_rules(&self) -> Outcome {
        if self.markers_placed < 3 {
            return Outcome::Undecided;
        } else {
            let game = winning_game(&self.state);
            let (won, _) = game;

            match game {
                (true, marker) => Outcome::Win(marker),
                (false, _) => {
                    if self.markers_placed == 9 && !won {
                        return Outcome::Draw;
                    } else {
                        return Outcome::Undecided;
                    }
                }
            }
        }
    }

    pub fn example_board() -> String {
        format!(
            "
        +-----+-----+------+\n
        |  1  |  2  |   3  |\n
        +-----+------+-----+\n
        +-----+------+-----+\n
        |  4  |  5   |  6  |\n
        +-----+------+-----+\n
        +-----+------+-----+\n
        |  7  |  8   |  9  |\n
        +-----+------+-----+\n"
        )
    }
}

fn winning_game(board_state: &[Marker; 9]) -> (bool, Marker) {
    // row search O(1) baby, who cares if it's ugly :)
    if board_state[0] == board_state[1] && board_state[1] == board_state[2] {
        return (true, board_state[1]);
    } else if board_state[3] == board_state[4] && board_state[4] == board_state[5] {
        return (true, board_state[4]);
    } else if board_state[6] == board_state[7] && board_state[7] == board_state[8] {
        return (true, board_state[7]);
    }
    // column search
    else if board_state[0] == board_state[3] && board_state[3] == board_state[6] {
        return (true, board_state[0]);
    } else if board_state[1] == board_state[4] && board_state[4] == board_state[7] {
        return (true, board_state[1]);
    } else if board_state[2] == board_state[5] && board_state[5] == board_state[8] {
        return (true, board_state[2]);
    }
    // diagonal search
    else if board_state[0] == board_state[4] && board_state[4] == board_state[8] {
        return (true, board_state[0]);
    } else if board_state[2] == board_state[4] && board_state[4] == board_state[6] {
        return (true, board_state[3]);
    } else {
        return (false, Marker::Empty);
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board_state = &self.state;

        let pretty_board = format!(
            "
        +------+------+------+\n
        |  {}  |  {}  |  {}  |\n
        +------+------+------+\n
        |  {}  |  {}  |  {}  |\n
        +------+------+------+\n
        |  {}  |  {}  |  {}  |\n
        +------+------+------+",
            board_state[0],
            board_state[1],
            board_state[2],
            board_state[3],
            board_state[4],
            board_state[5],
            board_state[6],
            board_state[7],
            board_state[8],
        );

        write!(f, "{}", pretty_board)
    }
}

// todo: test game rules/ logic
// it would be nice to experiment with fuzz testing and the proptest
// api, overkill but would be good for learning
#[cfg(test)]
mod tests {
    fn test_thing() {}
}
