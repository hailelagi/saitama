// AI that chooses a position on the board
// According to the difficulty EASY or HARD
// easy - a simple ai that chooses a random valid position on the board
// hard - a min-max algorithm that chooses a valid position + alpha-beta pruning.

use crate::{
    board::{board::Board, marker::Marker},
    world::World,
};

trait Strategy {
    fn choose_position(world: &World, board: Board) -> Board;
}

pub struct SimpleAI;
pub struct MinMax;

impl Strategy for SimpleAI {
    fn choose_position(world: &World, board: Board) -> Board {
        return board;
    }
}

impl Strategy for MinMax {
    fn choose_position(world: &World, board: Board) -> Board {
        return board;
    }
}

struct Opponent {
    marker: Marker,
}

// impl Opponent {
//     fn make_move(&self, world: &World, board: Board) -> Board {
//         match world.difficulty {
//             Difficulty::Easy =>
//         }
//         return board
//     }
// }
