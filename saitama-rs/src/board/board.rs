use crate::board::marker::Marker;
use crate::world::World;
use std::collections::HashMap;

pub enum Outcome {
    Win(Marker),
    Undecided,
    Draw,
}

pub struct Board {
    pub state: HashMap<i32, Marker>,
    markers_placed: i32,
}

impl Board {
    pub fn build(_world: &World) -> Self {
        Board {
            state: HashMap::from([
                (1, Marker::Empty),
                (2, Marker::Empty),
                (3, Marker::Empty),
                (4, Marker::Empty),
                (5, Marker::Empty),
                (6, Marker::Empty),
                (7, Marker::Empty),
                (8, Marker::Empty),
                (9, Marker::Empty),
            ]),
            markers_placed: 0,
        }
    }

    pub fn place_position(&mut self, position: i32, marker: Marker) -> Option<Marker> {
        let board_state = &mut self.state;

        match board_state.get(&position) {
            Some(Marker::Empty) => {
                self.markers_placed += 1;
                board_state.insert(position, marker);
                Some(marker)
            },
            None | Some(_) => None
        }
    }

    pub fn validate_game_rules(&self) -> Outcome {
        let board_state = &self.state;
        let game = winning_game(board_state);
        let (won, _) = game;

        if self.markers_placed < 3 {
            return Outcome::Undecided;
        } else if self.markers_placed == 9 && !won {
            return Outcome::Draw;
        } else {
            match game {
                (true, marker) => Outcome::Win(marker),
                (false, _) => Outcome::Undecided,
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

fn winning_game(board_state: &HashMap<i32, Marker>) -> (bool, Marker) {
    // row search O(1) baby, who cares if it's ugly :)
    if board_state[&1] == board_state[&2] && board_state[&2] == board_state[&3] {
        return (true, board_state[&1]);
    } else if board_state[&4] == board_state[&5] && board_state[&5] == board_state[&6] {
        return (true, board_state[&4]);
    } else if board_state[&7] == board_state[&8] && board_state[&8] == board_state[&9] {
        return (true, board_state[&7]);
    }
    // column search
    else if board_state[&1] == board_state[&4] && board_state[&4] == board_state[&7] {
        return (true, board_state[&1]);
    } else if board_state[&2] == board_state[&5] && board_state[&5] == board_state[&8] {
        return (true, board_state[&2]);
    } else if board_state[&3] == board_state[&6] && board_state[&6] == board_state[&9] {
        return (true, board_state[&3]);
    }
    // diagonal search
    else if board_state[&1] == board_state[&5] && board_state[&5] == board_state[&9] {
        return (true, board_state[&1]);
    } else if board_state[&3] == board_state[&5] && board_state[&5] == board_state[&7] {
        return (true, board_state[&3]);
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
            board_state[&1],
            board_state[&2],
            board_state[&3],
            board_state[&4],
            board_state[&5],
            board_state[&6],
            board_state[&7],
            board_state[&8],
            board_state[&9]
        );

        write!(f, "{}", pretty_board)
    }
}

#[cfg(test)]
mod tests {
    fn test_thing() {}
}
