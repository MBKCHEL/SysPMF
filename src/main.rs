mod player;
mod scanner;
use directories::UserDirs;
use rodio;
use std::fs;
use std::io::stdin;

fn main() {
    println!("SysPMF v0.1.0 by MBKCHEL | Type 'h' or 'help' for commands");

    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = rodio::Player::connect_new(&handle.mixer());

    let mut volume: f32 = 0.5;
    player.set_volume(volume);

    if let Some(user_dirs) = UserDirs::new() {
        let path = user_dirs.home_dir().join("SysPMF");
        if let Err(e) = fs::create_dir_all(&path) {
            eprintln!("Error creating directory: {e}");
        }

        let playlist = scanner::find_audio_files(&path);
        let mut current_index: usize = 0;

        if !playlist.is_empty() {
            println!("--- Playlist ---");
            for (i, track) in playlist.iter().enumerate() {
                let file_name = track.file_name().unwrap_or_default();
                println!("{}. {:?}", i + 1, file_name);
            }
            println!("----------------");
        }

        println!("Found audio files in SysPMF: {}", playlist.len());

        if !playlist.is_empty() {
            player::play_playlist(&playlist[current_index..], &player);
        }

        player.pause();

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
                    if player.empty() && !playlist.is_empty() {
                        player::play_playlist(&playlist[current_index..], &player);
                    } else {
                        player.play();
                    }
                    println!("Turn on");
                }
                "s" | "pause" => {
                    player.pause();
                    println!("Turn off");
                }
                "n" | "next" => {
                    if !playlist.is_empty() {
                        current_index = (current_index + 1) % playlist.len();
                        player::play_playlist(&playlist[current_index..], &player);
                    }
                }
                "b" | "back" => {
                    if !playlist.is_empty() {
                        if current_index == 0 {
                            current_index = playlist.len() - 1;
                        } else {
                            current_index -= 1;
                        }
                        player::play_playlist(&playlist[current_index..], &player);
                    }
                }
                "-" | "low" => {
                    volume = (volume - 0.1).max(0.0);
                    player.set_volume(volume);
                    println!("decrease (current: {:.1})", volume);
                }
                "+" | "high" => {
                    volume = (volume + 0.1).min(1.0);
                    player.set_volume(volume);
                    println!("increase (current: {:.1})", volume);
                }
                _ => println!("missing command"),
            }
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
        "b or back - play previous music",
        "- or low - decrease volume",
        "+ or high - increase volume",
        "Audio directory: ~/SysPMF (or C:/Users/<User>/SysPMF)",
        "Place your audio files in ~/SysPMF",
    ];

    for element in help_print {
        println!("{element}");
    }
}
