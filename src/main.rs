use rodio;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::stdin;

fn main() {
    println!("SysPMF v0.1.0 by MBKCHEL | Type 'h' or 'help' for commands");

    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");

    let player = rodio::Player::connect_new(&handle.mixer());

    let file = File::open("MDMA.mp3").unwrap();

    let source = rodio::Decoder::new(BufReader::new(file)).expect("error");

    player.append(source);
    player.pause();

    let mut volume: f32 = 0.5;

    let path = dirs::home_dir()
        .expect("Error, not found folder home ")
        .join("SysPMF");

    match fs::create_dir_all(&path) {
        Ok(_) => print!(""),
        Err(e) => eprintln!("Error {e}"),
    };

    loop {
        let mut user_input = String::new();

        stdin().read_line(&mut user_input).expect("error");

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

            "-" | "low" => {
                volume = (volume - 0.1).max(0.0);
                player.set_volume(volume);
                println!("decrease");
            }
            "+" | "high" => {
                volume = (volume + 0.1).max(0.0);
                player.set_volume(volume);
                println!("increase");
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
        "- or low - decrease volume",
        "+ or high - increase volume",
        "Audio directory: ~/SysPMF (or C:/Users/<User>/SysPMF)",
        "Place your audio files in ~/SysPMF",
    ];

    for element in help_print {
        println!("{element}");
    }
}
