// must read and write to STDOUT using native APIs
// Implement a game of tic tac toe in two modes - grid and n x n grid
// TODO: tests :)

use crate::board::Board;
use crate::world::World;

pub mod board;
pub mod intro;
pub mod world;

fn main() {
    intro::title_message();

    loop {
        match render_world() {
            Ok(world) => {
                render_board(world);
                continue;
            }

            Err(e) => {
                world::output_message(e.to_string().as_str());
                continue;
            }
        }
    }

    // panic!("replace me with a proper exit")
}

fn render_world<'a>() -> Result<world::World<'a>, std::io::Error> {
    let player_marker = intro::marker_choice()?;
    let game_world = World::select_difficulty(&player_marker)?;
    let confirm_message = format!("You're playing on {} mode :)", game_world.difficulty);

    world::output_message(&confirm_message);

    Ok(game_world)
}

fn render_board<'a>(world: World) -> Result<world::World<'a>, std::io::Error> {
    let board = Board::build(world)?;
    let player_marker = board::choose_position(board)?;

    Ok(game_world)
}
