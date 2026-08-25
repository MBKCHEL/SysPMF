use directories::UserDirs;
use rodio;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::stdin;
use std::path::{Path, PathBuf};

fn find_audio_files<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    let mut audio_files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() {
                match path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_lowercase())
                {
                    Some(ext) if ext == "mp3" || ext == "ogg" || ext == "wav" => {
                        audio_files.push(path);
                    }
                    _ => {}
                }
            }
        }
    }

    audio_files
}

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

        let playlist = find_audio_files(&path);
        println!("Found audio files in SysPMF: {}", playlist.len());

        for track_path in playlist {
            if let Ok(file) = File::open(&track_path) {
                let reader = BufReader::new(file);
                if let Ok(source) = rodio::Decoder::new(reader) {
                    player.append(source);
                    println!(
                        "Added to queue: {:?}",
                        track_path.file_name().unwrap_or_default()
                    );
                }
            };
        }
    } else {
        eprintln!("Error! Could not find home directories");
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
                player.play();
                println!("Turn on");
            }
            "s" | "pause" => {
                player.pause();
                println!("Turn off");
            }
            "n" | "next" => {
                player.skip_one();
                println!("Next track");
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
