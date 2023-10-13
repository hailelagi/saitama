use std::io::{self, Write};
use crate::world;

const TITLE_MESSAGE: &[u8] = b"Hi, welcome to Tic Tac Toe! would you like to be X or O?\n >>> ";
const INTRO_MESSAGE: &str = "Please select a difficulty! \n 1. HARD or 'H' \n 2. EASY or 'H'";

pub fn title_message() {
    // It's okay to panic if STDIN/STDERR is unavailable
    // TODO: why does writing to stderr happen before stdout?
    return io::stderr()
        .write_all(TITLE_MESSAGE)
        .expect("cannot write to STDERR for some reason :(");
}

pub fn marker_choice() -> io::Result<String> {
    let mark =world::player_input()?;

    match mark.as_str() {
        "X" | "O" => {
            let message = format!("You are now player {mark} - {INTRO_MESSAGE}");
            world::output_message(&message);
            Ok(mark.to_string())
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "did you mean X or O?",
        )),
    }
}
