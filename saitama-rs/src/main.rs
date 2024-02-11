// must read and write to STDOUT using native APIs
// Implement a game of tic tac toe with a min max ai

// https://stackoverflow.com/questions/41820114/hashing-every-combination-of-tic-tac-toe-table
// https://leetcode.com/problems/find-winner-on-a-tic-tac-toe-game/

use crate::board::{settings::Settings, Board, Outcome};

pub mod board;
pub mod decision;
pub mod repl;

fn main() {
    repl::title_message();

    loop {
        match render_world() {
            Ok(()) => outro(),
            Err(e) => {
                repl::output_message(e.to_string().as_str());
                continue;
            }
        }
    }
}

fn render_world() -> Result<(), std::io::Error> {
    let game_world: Settings = render_intro()?;
    let board_session = render_session(game_world)?;

    Ok(board_session)
}

fn render_intro() -> Result<Settings, std::io::Error> {
    let marker = repl::marker_choice()?;
    let difficulty = repl::select_difficulty()?;
    let game_world = Settings::build(difficulty, marker);

    let confirm_message = format!("You're playing on {} mode :)", game_world.difficulty);

    repl::output_message(&confirm_message);

    Ok(game_world)
}

fn render_session(settings: Settings) -> Result<(), std::io::Error> {
    let mut board = Board::new(&settings);
    let example_board = format!("{}", Board::example_board());
    repl::output_message(&example_board);

    loop {
        let player_position = match repl::position_input() {
            Ok(p) => p,
            Err(e) => {
                repl::output_message(e.to_string().as_str());
                continue;
            }
        };

        Board::place_position(&mut board, player_position, settings.player_marker);

        if let Some(ai_position) = decision::choose_position(board.settings.difficulty, &board) {
            Board::place_position(&mut board, ai_position, settings.opponent_marker);
        } else {
            repl::output_message("ai cannot choose a position!");
        }

        match board.validate_game_rules() {
            Outcome::Draw => {
                repl::output_message(&format!("{}", &board));
                repl::output_message("the game ends as a draw!");
                break Ok(());
            }
            Outcome::Win(winner) => {
                repl::output_message(&format!("{}", &board));
                repl::output_message(&format!("{} is the winner!", winner));
                break Ok(());
            }
            Outcome::Undecided => {
                repl::output_message(&format!("{}", &board));
                continue;
            }
        }
    }
}

fn outro() {
    todo!("replace me with a proper exit");
    std::process::exit(0)
}
