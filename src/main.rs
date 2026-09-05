mod player;
mod scanner;
use directories::UserDirs;
use rodio;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn play_current_track(playlist: &[PathBuf], index: usize, player: &rodio::Player) {
    if playlist.is_empty() {
        return;
    }
    let track = &playlist[index];
    if player::play_track(track, player) {
        let file_name = track
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown Track");
        println!("Now playing [{}]: {}", index + 1, file_name);
    } else {
        println!("❌ Error playing track: {:?}", track);
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

        let playlist = scanner::find_audio_files(&path);
        let mut current_index: usize = 0;
        let mut is_paused = true;

        if !playlist.is_empty() {
            println!("--- Playlist ---");
            for (i, track) in playlist.iter().enumerate() {
                let file_name = track
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown");
                println!("{}. {}", i + 1, file_name);
            }
            println!("----------------");
        }

        println!("Found audio files in SysPMF: {}", playlist.len());

        if !playlist.is_empty() {
            play_current_track(&playlist, current_index, &player);
        }

        player.pause();

        let (tx, rx) = mpsc::channel::<String>();

        thread::spawn(move || {
            loop {
                let mut user_input = String::new();
                if std::io::stdin().read_line(&mut user_input).is_ok() {
                    let cmd = user_input.to_lowercase().trim().to_string();
                    if tx.send(cmd).is_err() {
                        break;
                    }
                }
            }
        });

        loop {
            if !playlist.is_empty() && player.empty() && !is_paused {
                current_index = (current_index + 1) % playlist.len();
                play_current_track(&playlist, current_index, &player);
            }
            if let Ok(command) = rx.try_recv() {
                match command.as_str() {
                    "q" | "quit" => {
                        println!("leave");
                        break;
                    }
                    "h" | "help" => help(),
                    "p" | "play" => {
                        is_paused = false;
                        if player.empty() && !playlist.is_empty() {
                            play_current_track(&playlist, current_index, &player);
                        } else {
                            player.play();
                            println!("Turn on");
                        }
                    }
                    "s" | "pause" => {
                        is_paused = true;
                        player.pause();
                        println!("Turn off");
                    }
                    "n" | "f" | "next" | "forward" => {
                        if !playlist.is_empty() {
                            is_paused = false;
                            current_index = (current_index + 1) % playlist.len();
                            play_current_track(&playlist, current_index, &player);
                        }
                    }
                    "b" | "back" => {
                        if !playlist.is_empty() {
                            is_paused = false;
                            if current_index == 0 {
                                current_index = playlist.len() - 1;
                            } else {
                                current_index -= 1;
                            }
                            play_current_track(&playlist, current_index, &player);
                        }
                    }
                    "-" | "l" | "low" => {
                        volume = (volume - 0.1).max(0.0);
                        player.set_volume(volume);
                        println!("decrease (current: {:.1})", volume);
                    }
                    "+" | "u" | "high" => {
                        volume = (volume + 0.1).min(1.0);
                        player.set_volume(volume);
                        println!("increase (current: {:.1})", volume);
                    }
                    "" => {}
                    _ => println!("missing command"),
                }
            }
            thread::sleep(Duration::from_millis(50));
        }
    }
}

fn help() {
    let help_print = [
        "h or help - print all command",
        "q or quit - leave",
        "s or pause - stop play music",
        "p or play - play music",
        "n, next, f, forward - play next music",
        "b or back - play previous music",
        "-, low, l - decrease volume",
        "+, high, u - increase volume",
        "Audio directory: ~/SysPMF (or C:/Users/<User>/SysPMF)",
        "Place your audio files in ~/SysPMF",
    ];

    for element in help_print {
        println!("{element}");
    }
}
