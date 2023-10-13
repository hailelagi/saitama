use std::io::{self, Write};

// Config struct to build and render tic-tac-toe game universe and session
#[derive(Debug)]
pub struct World<'a> {
    pub difficulty: &'a str,
    pub player_marker: String,
    // TODO: store an rng to determine how difficult the ai will be
}

impl World<'static> {
    pub fn build<'a>(difficulty: &'a str, marker: String) -> World<'a> {
        World {
            difficulty,
            player_marker: marker,
        }
    }

    pub fn select_difficulty(marker: &str) -> io::Result<World<'static>> {
        loop {
            let difficulty_choice = player_input()?;

            return match difficulty_choice.as_str() {
                "EASY" | "E" => Ok(World::build("EASY", String::from(marker))),
                "HARD" | "H" => Ok(World::build("HARD", String::from(marker))),
                _ => {
                    output_message("did you mean EASY or HARD?");
                    continue;
                }
            };
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
