use rand::rngs::ThreadRng;
use std::io::{self, Error, Write};
use crate::board::marker::Marker;

// Config struct to build and render tic-tac-toe game universe and session
#[derive(Debug)]
pub struct World {
    pub difficulty: Difficulty,
    pub player_marker: Marker,
    pub opponent_marker: Marker,
    pub seed: ThreadRng,
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
        let opponent_marker = match player_marker {
            Marker::X => Marker::O,
            Marker::O => Marker::X,
            Marker::Empty => Marker::Empty
        };
        
        World {
            difficulty,
            player_marker,
            opponent_marker,
            seed: rand::thread_rng(),
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

pub fn position_input() -> io::Result<usize> {
    output_message("Please input a number from 1 - 9 to select a position on the grid");

    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer)?;

    // this pattern is not idiomatic, there are third party libraries
    // which handle this much better e.g anyhow::Error for a large project
    // this will have to do
    match input_buffer.trim().parse::<usize>() {
        Ok(n) => Ok(n),
        Err(e) => Err(Error::new(io::ErrorKind::InvalidInput, e.to_string())),
    }
}

pub fn output_message(s: &str) {
    let message = String::from(s) + "\n >>> ";

    io::stderr()
        .lock()
        .write_all(message.as_bytes())
        .expect("cannot write to STDOUT for some reason :(")
}
