// must read and write to STDOUT using native APIs
// Implement a game of tic tac toe in two modes - grid and n x n grid

use crate::world::World;

pub mod intro;
pub mod world;

fn main() {
    intro::title_message();

    loop {
        match render_world() {
            Ok(world) => {
                let message = format!("You're playing on {} mode :)", world.difficulty);
                world::output_message(&message);
            }

            Err(e) => {
                world::output_message(e.to_string().as_str());
                continue;
            }
        }
    }

    // panic!("replace me with a proper exit")
}

pub fn render_world<'a>() -> Result<world::World<'a>, std::io::Error> {
    let player_marker = intro::marker_choice()?;
    let game_world = World::select_difficulty(&player_marker);

    println!("debug: {:?}", game_world);
    return game_world
}
