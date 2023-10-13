// must read and write to STDOUT using native APIs
// Implement a game of tic tac toe in two modes - grid and n x n grid
//
pub mod intro;

fn main() {
    intro::title_message();

    loop {
        render_world();
    }

    // panic!("replace me with a proper exit")
}

fn render_world() -> Result<String, std::io::Error> {
    let player_marker = intro::player_input()?;
    let difficulty = intro::select_difficulty(&player_marker)?;

    println!("debug: {:?}", difficulty);

    return Ok(String::from("wip"));
}
