use std::io::{self, Write};

use anyhow::{anyhow, Result};

use crate::board::marker::Marker;
use crate::board::settings::Difficulty;

const TITLE_MESSAGE: &[u8] = b"Hi, welcome to Tic Tac Toe! would you like to be X or O?\n >>> ";
const DIFFICULTY_MESSAGE: &str = "Please select a difficulty! \n 1. HARD or 'H' \n 2. EASY or 'E'";

pub fn select_difficulty() -> Result<Difficulty> {
    loop {
        let difficulty_choice = player_input()?;

        return match difficulty_choice.as_str() {
            "EASY" | "E" => Ok(Difficulty::Easy),
            "HARD" | "H" => Ok(Difficulty::HardEnum),
            _ => {
                output_message("did you mean EASY or HARD?");
                continue;
            }
        };
    }
}

pub fn title_message() {
    // It's okay to panic if STDIN/STDERR is unavailable
    return io::stderr()
        .write_all(TITLE_MESSAGE)
        .expect("cannot write to STDERR for some reason :(");
}

pub fn player_input() -> Result<String> {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer)?;

    let input = input_buffer.to_uppercase();
    let input = input.trim();

    Ok(input.to_owned())
}

pub fn position_input() -> Result<usize> {
    output_message("Please input a number from 1 - 9 to select a position on the grid");

    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer)?;

    match input_buffer.trim().parse::<usize>() {
        Ok(n) => validate_board_range(n),
        Err(e) => Err(e.into()),
    }
}
pub fn marker_choice() -> Result<Marker> {
    let player_mark = player_input()?;
    let mark = Marker::new(&player_mark)?;

    let message = format!("You are now player {player_mark} - {DIFFICULTY_MESSAGE}");
    output_message(&message);

    Ok(mark)
}

pub fn output_message(s: &str) {
    let message = String::from(s) + "\n >>> ";

    io::stderr()
        .lock()
        .write_all(message.as_bytes())
        .expect("cannot write to STDOUT for some reason :(")
}

fn validate_board_range(n: usize) -> Result<usize> {
    if n >= 1 && n <= 9 {
        return Ok(n - 1);
    } else {
        Err(anyhow!("position not on grid: {}", n))
    }
}
