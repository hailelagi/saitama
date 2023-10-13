use std::{
    fmt::format,
    io::{self, Write},
};

const INTRO_MESSAGE: &[u8] = b"Hi, welcome to Tic Tac Toe! would you like to be X or O?\n >>> ";

// Config struct to build and render tic-tac-toe session

#[derive(Debug)]
pub struct World<'a> {
    pub difficulty: &'a str,
    pub player_marker: String,
    // TODO: store an rng to determine how difficult the ai will be
}

impl World<'static> {
    pub fn new<'a>(difficulty: &'a str, marker: String) -> World<'a> {
        World {
            difficulty,
            player_marker: marker,
        }
    }
}

struct MarkerChoice<'a>(&'a str, &'a str);

pub fn title_message() {
    // It's okay to panic if STDIN/STDERR is unavailable
    // TODO: why does writing to stderr happen before stdout?
    return io::stderr()
        .write_all(INTRO_MESSAGE)
        .expect("cannot write to STDERR for some reason :(");
}

pub fn select_difficulty(marker: &str) -> io::Result<World<'static>> {
    loop {
        let mut input_buffer = String::new();
        io::stdin().read_line(&mut input_buffer)?;

        let input = input_buffer.to_uppercase();
        let input = input.trim();

        let difficulty = match input {
            "EASY" | "E" => Ok(World::new("EASY", String::from(marker))),
            "HARD" | "H" => Ok(World::new("HARD", String::from(marker))),
            _ => Err(io::ErrorKind::InvalidInput),
        };

        match difficulty {
            Ok(world) => {
                let message = format!("You're playing on {} mode :)", world.difficulty);
                output_message(&message);

                return Ok(world);
            }

            Err(e) => {
                output_message(e.to_string().as_str());
                continue;
            }
        }
    }
}

// pub fn marker_choice() -> {

// }

pub fn player_input() -> io::Result<String> {
    const INTRO_MESSAGE: &str = "Please select a difficulty! \n 1. HARD or 'H' \n 2. EASY or 'H'";

    loop {
        let mut input_buffer = String::new();

        io::stdin().read_line(&mut input_buffer)?;

        let input = input_buffer.to_uppercase();
        let mark = input.trim();

        return match mark {
            "X" | "O" => {
                let message = format!("You are now player {mark} - {INTRO_MESSAGE}");
                output_message(&message);
                Ok(mark.to_string())
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "did you mean X or O?",
            )),
        };
    }
}

fn output_message(s: &str) {
    let message = String::from(s) + "\n >>> ";

    io::stderr()
        .lock()
        .write_all(message.as_bytes())
        .expect("cannot write to STDOUT for some reason :(")
}
