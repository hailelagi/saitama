use std::fmt::format;
// EASY MODE: Implement a 3 x 3 board grid
// HARD MODE: Implement an n x n board
use std::io;

use crate::board::Marker::Empty;
use crate::world::Difficulty;
use crate::world::World;

#[derive(Debug, Clone, Copy)]
pub enum Marker {
    X,
    O,
    Empty,
}

impl Marker {
    pub fn new(mark: &str) -> io::Result<Self> {
        match mark {
            "X" => Ok(Marker::X),
            "O" => Ok(Marker::O),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "did you mean X or O?",
            )),
        }
    }
}

impl std::fmt::Display for Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Marker::O => write!(f, "({})", "O"),
            Marker::X => write!(f, "({})", "X"),
            Marker::Empty => write!(f, "({})", ""),
        }
    }
}

#[derive(Debug)]
pub enum Board {
    Grid {
        row_one: (Marker, Marker, Marker),
        row_two: (Marker, Marker, Marker),
        row_three: (Marker, Marker, Marker),
    },
    Dynamic,
}

impl Board {
    pub fn build(world: &World) -> Self {
        match world.difficulty {
            Difficulty::Easy => Board::Grid {
                row_one: (Empty, Empty, Empty),
                row_two: (Empty, Empty, Empty),
                row_three: (Empty, Empty, Empty),
            },
            Difficulty::Hard => Board::Dynamic,
        }
    }

    pub fn place_position(self, position: String, marker: Marker) -> io::Result<Self> {
        // TODO: validate board rules
        Ok(Board::Dynamic)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Board::Grid {
                row_one,
                row_two,
                row_three,
            } => {
                let pretty_board = format!(
                    "
        +------+-----+-------+\n
        |  {}  |  {} |   {}  |\n
        +------+-----+-------+\n
        ",
                    row_one.0, row_one.1, row_one.2
                );

                write!(f, "{}", pretty_board)
            }
            _ => write!(f, "todo"),
        }
    }
}
