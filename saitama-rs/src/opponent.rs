// AI that chooses a position on the board
// According to the difficulty EASY or HARD
// easy - a simple ai that chooses a random valid position on the board
// hard - a min-max algorithm that chooses a valid position

use crate::{
    board::{board::Board, marker::Marker},
    world::World,
};

trait Strategy {
    fn choose_position(world: &World, board: Board) -> Board {}
}

struct SimpleAI;
struct MinMax;

// impl Strategy for SimpleAI {
    
// }

// impl Strategy for MinMax {

// }
  

struct Opponent {
    marker: Marker,
    strategy: Strategy,
}

impl Opponent {
    fn make_move(&self, world: &World, board: Board) -> Board {
        match world.difficulty {
            Difficulty::Easy => 
        }
        return board
    }
}
