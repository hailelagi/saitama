// must read and write to STDOUT using native APIs
pub mod intro;

fn main() {
    intro::title_message();

    loop {
        let _position = match intro::player_input() {
            Ok(position) => {
                println!("{}", position);
                break;
            }
            Err(_) => panic!("bad input"),
        };
    }

    println!("carry on soldier")
}

