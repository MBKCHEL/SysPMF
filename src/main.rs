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

fn play_track(playlist: &[PathBuf], index: usize, player: &rodio::Player)  {
     if let Some(track_path) = playlist.get(index) {
         player.stop();
         if let Ok(file) = File::open(&track_path) {
             let reader = BufReader::new(file);
             if let Ok(source) = rodio::Decoder::new(reader) {
                 player.append(source);
                 player.play();
                 println!(
                     "Selected track: {:?}",
                     track_path.file_name().unwrap_or_default()
                 );
             }
         }
     }
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
            play_track(&playlist, current_index, &player);
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
                        play_track(&playlist, current_index, &player);
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
                        play_track(&playlist, current_index, &player);
                    }
                }
                "b" | "back" => {
                    if !playlist.is_empty() {
                        if current_index == 0 {
                            current_index = playlist.len() - 1;
                        } else {
                            current_index -= 1;
                        }
                        play_track(&playlist, current_index, &player);
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
