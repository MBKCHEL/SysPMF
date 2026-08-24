use rodio;
use std::fs::File;
use std::io::BufReader;
use std::io::stdin;

fn main() {
    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");

    let player = rodio::Player::connect_new(&handle.mixer());

    let file = File::open("MDMA.mp3").unwrap();

    let source = rodio::Decoder::new(BufReader::new(file)).expect("error");

    player.append(source);
    player.pause();

    loop {
        let mut user_input = String::new();

        stdin().read_line(&mut user_input).expect("error");

        println!("{user_input}");

        match user_input.to_lowercase().as_str().trim() {
            "q" | "quit" => {
                println!("leave");
                break;
            }
            "h" | "help" => help(),
            "p" | "play" => {
                player.play();
                println!("Turn on");
            }
            "s" | "pause" => {
                player.pause();
                println!("Turn off");
            }
            _ => println!("missing command"),
        }
    }
}

fn help() {
    let help_print = [
        "h or help - print all command",
        "q or quit - leave",
        "s or pause - stop play music",
        "p or play - play music",
        "n or next - play next music",
    ];

    for element in help_print {
        println!("{element}");
    }
}
