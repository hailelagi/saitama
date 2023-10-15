// must read and write to STDOUT using native APIs
// Implement a game of tic tac toe in two modes - grid and n x n grid
// TODO: tests :)

use crate::board::Board;

pub mod board;
pub mod intro;
pub mod world;

fn main() {
    intro::title_message();

    loop {
        match render_world() {
            Ok(world) => {
                let board = Board::build(&world);
                println!("DEBUG: {:?}", board);
                // render_session(world.player_marker, board);
                outro();
                
                break;
            }

            Err(e) => {
                world::output_message(e.to_string().as_str());
                continue;
            }
        }
    }

    // panic!("replace me with a proper exit")
}

fn render_world<'a>() -> Result<world::World, std::io::Error> {
    let player_marker = intro::marker_choice()?;
    let game_world = intro::select_difficulty(player_marker)?;
    let confirm_message = format!("You're playing on {} mode :)", game_world.difficulty);

    world::output_message(&confirm_message);

    Ok(game_world)
}

// fn render_session<'a>(marker: Marker, board: Board) -> Result<world::World, std::io::Error> {
//     let board = Board::choose_position(board, marker)?;
//     let board = Board::validate_rules(board);

//     Ok(board)
// }

fn outro() {
    panic!("replace me with a proper exit")
}