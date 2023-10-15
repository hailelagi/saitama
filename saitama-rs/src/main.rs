// must read and write to STDOUT using native APIs
// Implement a game of tic tac toe in two modes - grid and n x n grid
// TODO: tests :)

use std::marker;

use crate::board::{Board, Marker};

pub mod board;
pub mod intro;
pub mod world;

fn main() {
    intro::title_message();

    loop {
        match render_world() {
            Ok(()) => outro(),
            Err(e) => {
                world::output_message(e.to_string().as_str());
                continue;
            }
        }
    }
}

fn render_world() -> Result<(), std::io::Error> {
    let game_world = render_intro()?;
    let board_session = render_session(game_world)?;

    Ok(board_session)
}

fn render_intro() -> Result<world::World, std::io::Error> {
    let marker = intro::marker_choice()?;
    let game_world = intro::select_difficulty(marker)?;

    let confirm_message = format!("You're playing on {} mode :)", game_world.difficulty);

    world::output_message(&confirm_message);

    Ok(game_world)
}

fn render_session(world: world::World) -> Result<(), std::io::Error> {
    let board = Board::build(&world);

    println!("DEBUG: {}", board);

    loop {
        let position = world::player_input()?;
        let board = Board::place_position(board, position, world.player_marker)?;
        break;
    }

    Ok(())
}

fn outro() {
    panic!("replace me with a proper exit")
}
