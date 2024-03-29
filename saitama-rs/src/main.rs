// must read and write to STDOUT using native APIs
// Implement a game of tic tac toe with a min max ai

// https://stackoverflow.com/questions/41820114/hashing-every-combination-of-tic-tac-toe-table
// https://leetcode.com/problems/find-winner-on-a-tic-tac-toe-game/

use crate::board::{settings::Settings, Board, Outcome};

use anyhow::Result;

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

fn render_world() -> Result<()> {
    let init = render_intro()?;
    let board_session = render_session(init)?;

    Ok(board_session)
}

fn render_intro() -> Result<Settings> {
    let marker = repl::marker_choice()?;
    let difficulty = repl::select_difficulty()?;
    let game_world = Settings::build(difficulty, marker);

    let confirm_message = format!("You're playing on {} mode :)", game_world.difficulty);

    repl::output_message(&confirm_message);

    Ok(game_world)
}

fn render_session(settings: Settings) -> Result<()> {
    let player_marker = settings.player_marker;
    let opponent_marker = settings.opponent_marker;

    let mut board = Board::new(settings);
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

        Board::place_position(&mut board, player_position, player_marker);

        if let Some(ai_position) = decision::choose_position(board.settings.difficulty, &board) {
            Board::place_position(&mut board, ai_position, opponent_marker);
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
    repl::output_message("Thanks for playing!");
    std::process::exit(0)
}
