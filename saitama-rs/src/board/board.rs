use crate::board::marker::{Marker, Marker::Empty};
use crate::world::Difficulty;
use crate::world::World;
use std::io;

pub struct Row(Marker, Marker, Marker);

pub enum Board {
    Grid {
        row_one: Row,
        row_two: Row,
        row_three: Row,
    },
    Dynamic,
}

impl Board {
    pub fn build(world: &World) -> Self {
        match world.difficulty {
            Difficulty::Easy => Board::Grid {
                row_one: Row(Empty, Empty, Empty),
                row_two: Row(Empty, Empty, Empty),
                row_three: Row(Empty, Empty, Empty),
            },
            Difficulty::Hard => Board::Dynamic,
        }
    }

    pub fn place_position(self, position: String, marker: Marker) -> io::Result<Self> {
        // find empty space on the board - place marker
        Ok(Board::Dynamic)
    }

    pub fn won_board(&self) -> bool {
        match self {
            Board::Grid {
                row_one,
                row_two,
                row_three,
            } => {
                let column_win = row_one == row_two && row_two == row_three;

                column_win || row_one.win() || row_two.win() || row_three.win()
            }

            Board::Dynamic => false,
        }
    }
}

impl Row {
    fn win(&self) -> bool {
        self.0 == self.1 && self.1 == self.2 && self.2 != Marker::Empty
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 || self.1 == other.1 || self.2 == other.2
    }
}

impl Eq for Row {}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Board::Grid {
                row_one,
                row_two,
                row_three,
            } => write!(f, "{}{}{}", row_one, row_two, row_three),
            Board::Dynamic => write!(f, "{}", "coming soon!"),
        }
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty_board = format!(
            "
        +------+------+------+\n
        |  {}  |  {}  |  {}  |\n
        +------+------+------+",
            self.0, self.1, self.2
        );

        write!(f, "{}", pretty_board)
    }
}
