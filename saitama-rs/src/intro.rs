use crate::board::Marker;
use crate::world;
use crate::world::{Difficulty, World};
use std::io::{self, Write};

const TITLE_MESSAGE: &[u8] = b"Hi, welcome to Tic Tac Toe! would you like to be X or O?\n >>> ";
const INTRO_MESSAGE: &str = "Please select a difficulty! \n 1. HARD or 'H' \n 2. EASY or 'H'";

pub fn select_difficulty(marker: Marker) -> io::Result<World> {
    loop {
        let difficulty_choice = world::player_input()?;

        return match difficulty_choice.as_str() {
            "EASY" | "E" => Ok(World::build(Difficulty::Easy, marker)),
            "HARD" | "H" => Ok(World::build(Difficulty::Hard, marker)),
            _ => {
                world::output_message("did you mean EASY or HARD?");
                continue;
            }
        };
    }
}
pub fn title_message() {
    // It's okay to panic if STDIN/STDERR is unavailable
    // TODO: why does writing to stderr happen before stdout?
    return io::stderr()
        .write_all(TITLE_MESSAGE)
        .expect("cannot write to STDERR for some reason :(");
}

pub fn marker_choice() -> io::Result<Marker> {
    let mark = world::player_input()?;
    let mark = Marker::new(&mark)?;

    let message = format!("You are now player {mark} - {INTRO_MESSAGE}");
    world::output_message(&message);

    Ok(mark)
}
