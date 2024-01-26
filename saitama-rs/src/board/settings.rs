use crate::board::marker::Marker;
use rand::rngs::ThreadRng;

// Config struct to build and render tic-tac-toe game universe and session
#[derive(Debug)]
pub struct Settings {
    pub difficulty: Difficulty,
    pub player_marker: Marker,
    pub opponent_marker: Marker,
    pub seed: ThreadRng,
}

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    HardSearchTree,
    HardEnum,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "<<{}>>", "easy"),
            Difficulty::HardEnum => write!(f, "<<{}>>", "hard enumerate all"),
            Difficulty::HardSearchTree => write!(f, "<<{}>>", "hard search tree pruning"),
        }
    }
}

impl Settings {
    pub fn build(difficulty: Difficulty, player_marker: Marker) -> Settings {
        let opponent_marker = match player_marker {
            Marker::X => Marker::O,
            Marker::O => Marker::X,
            Marker::Empty => Marker::Empty,
        };

        Settings {
            difficulty,
            player_marker,
            opponent_marker,
            seed: rand::thread_rng(),
        }
    }
}
