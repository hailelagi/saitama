use std::fmt::write;
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
            // todo: custom emoji markers!
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
        // TODO: validate board rules
        Ok(Board::Dynamic)
    }
}

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
