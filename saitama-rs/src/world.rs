use crate::board::marker::Marker;
use std::io::{self, Write};

// Config struct to build and render tic-tac-toe game universe and session
#[derive(Debug)]
pub struct World {
    pub difficulty: Difficulty,
    pub player_marker: Marker,
    // TODO: store an rng to determine how difficult the ai will be
}

#[derive(Debug)]
pub enum Difficulty {
    Easy,
    Hard,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "<<{}>>", "easy"),
            Difficulty::Hard => write!(f, "<<{}>>", "hard"),
        }
    }
}

impl World {
    pub fn build(difficulty: Difficulty, player_marker: Marker) -> World {
        World {
            difficulty,
            player_marker,
        }
    }
}

pub fn player_input() -> io::Result<String> {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer)?;

    let input = input_buffer.to_uppercase();
    let input = input.trim();

    Ok(input.to_owned())
}

pub fn output_message(s: &str) {
    let message = String::from(s) + "\n >>> ";

    io::stderr()
        .lock()
        .write_all(message.as_bytes())
        .expect("cannot write to STDOUT for some reason :(")
}
