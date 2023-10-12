use std::io::{self, Write};

struct PlayerChoice<'a>(&'a str, &'a str);

pub fn title_message() {
    // It's okay to panic if STDIN/STDERR is unavailable
    const INTRO_MESSAGE: &[u8] = b"Hi, welcome to Tic Tac Toe! would like to be X or O?\n";

    return io::stderr()
        .write_all(INTRO_MESSAGE)
        .expect("cannot write to STDERR for some reason :(");
}

pub fn player_input() -> io::Result<String> {
    let mut input_buffer = String::new();

    loop {
        io::stdin().read_line(&mut input_buffer)?;

        let input = input_buffer.as_str();

        let choice = match input {
            "X" => Ok(PlayerChoice(input, "You are now player 1!")),
            "O" => Ok(PlayerChoice(input, "You are now player 2!")),
            _ => Err(io::ErrorKind::InvalidInput),
        };

        match choice {
            Ok(PlayerChoice(mark, msg)) => {
                output_message(msg);
                return Ok(String::from(mark))
            }
            
            Err(e) => {
                let err_out = e.to_string() + "\n";

                output_message(&err_out);

            }
                
        }
    }

    // return Ok(String::from(""));
}


fn output_message(s: &str) {
    io::stderr()
        .lock()
        .write_all(s.as_bytes())
        .expect("cannot write to STDOUT for some reason :(")
}
