// must read and write to STDOUT using native APIs
// Implement a game of tic tac toe with a min max ai

// https://stackoverflow.com/questions/41820114/hashing-every-combination-of-tic-tac-toe-table
// https://leetcode.com/problems/find-winner-on-a-tic-tac-toe-game/

use crate::{board::board::Board, world::World};
use board::board::Outcome;
// use crate::opponent::{MinMax, SimpleAI};

pub mod board;
pub mod intro;
pub mod world;
pub mod opponent;

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
    let game_world: World = render_intro()?;
    let board_session = render_session(game_world)?;

    Ok(board_session)
}

fn render_intro() -> Result<world::World, std::io::Error> {
    let marker = intro::marker_choice()?;
    let difficulty = intro::select_difficulty()?;
    let game_world = World::build(difficulty, marker);

    let confirm_message = format!("You're playing on {} mode :)", game_world.difficulty);

    world::output_message(&confirm_message);

    Ok(game_world)
}

fn render_session(world: world::World) -> Result<(), std::io::Error> {
    let mut board = Board::build(&world);
    let example_board = format!("{}", Board::example_board());
    world::output_message(&example_board);

    loop {
        let position = world::position_input()?;
        Board::place_position(&mut board, position, world.player_marker);

        match world.difficulty {
            world::Difficulty::Easy => {
                // position = SimpleAI::choose_position(world, board);
                let position = 0 as usize;
                Board::place_position(&mut board, position, world.opponent_marker);
            }
            world::Difficulty::Hard => {
               //  position = MinMax::choose_position(world, board);
                let position = 0 as usize;
                Board::place_position(&mut board, position, world.opponent_marker);
            }
        }

        match board.validate_game_rules() {
            Outcome::Draw => {
                world::output_message("the game ends as a draw!");
                break Ok(());
            }
            Outcome::Win(winner) => {
                world::output_message(&format!("{} is the winner!", winner));
                break Ok(());
            }
            Outcome::Undecided => {
                world::output_message(&format!("{}", &board));
                continue;
            }
        }
    }
}

fn outro() {
    panic!("replace me with a proper exit")
}
