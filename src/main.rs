mod player;
mod scanner;
use directories::UserDirs;
use rodio;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::scanner::Playlist;

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
    println!("SysPMF v1.0.6 by MBKCHEL | Type 'h' or 'help' for commands");

    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = rodio::Player::connect_new(&handle.mixer());

    let mut volume: f32 = 0.5;
    player.set_volume(volume);

    if let Some(user_dirs) = UserDirs::new() {
        let path = user_dirs.home_dir().join("SysPMF");
        if let Err(e) = fs::create_dir_all(&path) {
            eprintln!("Error creating directory: {e}");
        }

        let mut playlists = scanner::scan_playlists(&path);
        if playlists.is_empty() {
            println!("No audio files found in ~/SysPMF!");
            return;
        }

        let mut current_playlist_idx: usize = 0;
        let mut current_track_idx: usize = 0;
        let mut is_paused = true;

        playlists_menu_print(&playlists);
        playlist_print(&playlists[current_playlist_idx]);

        if !playlists[current_playlist_idx].tracks.is_empty() {
            play_current_track(
                &playlists[current_playlist_idx].tracks,
                current_track_idx,
                &player,
            );
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
            let active_playlist = &playlists[current_playlist_idx];
            if !active_playlist.tracks.is_empty() && player.empty() && !is_paused {
                current_track_idx = (current_track_idx + 1) % active_playlist.tracks.len();
                play_current_track(&active_playlist.tracks, current_track_idx, &player);
            }

            if let Ok(command) = rx.try_recv() {
                let active_playlist = &playlists[current_playlist_idx];

                match command.as_str() {
                    "q" | "quit" | "й" => {
                        println!("leave");
                        break;
                    }
                    "h" | "help" | "р" => help(),
                    "pls" | "folders" => {
                        playlists_menu_print(&playlists);
                    }
                    cmd if cmd.starts_with("cd ") => {
                        if let Ok(num) = cmd.trim_start_matches("cd ").trim().parse::<usize>() {
                            if num > 0 && num <= playlists.len() {
                                current_playlist_idx = num - 1;
                                current_track_idx = 0;
                                is_paused = false;

                                let new_playlist = &playlists[current_playlist_idx];
                                println!("📁 Switched to: {}", new_playlist.name);
                                playlist_print(new_playlist);

                                if !new_playlist.tracks.is_empty() {
                                    play_current_track(
                                        &new_playlist.tracks,
                                        current_track_idx,
                                        &player,
                                    );
                                }
                            } else {
                                println!("❌ Invalid playlist number!");
                            }
                        }
                    }
                    "p" | "play" | "з" => {
                        is_paused = false;
                        if player.empty() && !active_playlist.tracks.is_empty() {
                            play_current_track(&active_playlist.tracks, current_track_idx, &player);
                        } else {
                            player.play();
                            println!("Turn on");
                        }
                    }
                    "s" | "pause" | "ы" => {
                        is_paused = true;
                        player.pause();
                        println!("Turn off");
                    }
                    "n" | "f" | "next" | "forward" | "т" => {
                        if !active_playlist.tracks.is_empty() {
                            is_paused = false;
                            current_track_idx =
                                (current_track_idx + 1) % active_playlist.tracks.len();
                            play_current_track(&active_playlist.tracks, current_track_idx, &player);
                        }
                    }
                    "b" | "back" | "и" => {
                        if !active_playlist.tracks.is_empty() {
                            is_paused = false;
                            if current_track_idx == 0 {
                                current_track_idx = active_playlist.tracks.len() - 1;
                            } else {
                                current_track_idx -= 1;
                            }
                            play_current_track(&active_playlist.tracks, current_track_idx, &player);
                        }
                    }
                    "-" | "l" | "low" => {
                        volume = (volume - 0.1).max(0.0);
                        player.set_volume(volume);
                        println!("decrease (current: {:.2})", volume);
                    }
                    "ml" | "micro-low" => {
                        volume = (volume - 0.01).max(0.0);
                        player.set_volume(volume);
                        println!("decrease (current: {:.2})", volume);
                    }
                    "+" | "u" | "high" => {
                        volume = (volume + 0.1).min(2.0);
                        player.set_volume(volume);
                        println!("increase (current: {:.2})", volume);
                    }
                    "mh" | "micro-high" => {
                        volume = (volume + 0.01).min(2.0);
                        player.set_volume(volume);
                        println!("increase (current: {:.2})", volume);
                    }
                    "ls" | "pl" | "list" => {
                        playlist_print(active_playlist);
                    }
                    "c" | "check" => {
                        playlists = scanner::scan_playlists(&path);

                        if playlists.is_empty() {
                            println!("⚠️ No audio files found after rescan!");
                        } else {
                            if current_playlist_idx >= playlists.len() {
                                current_playlist_idx = 0;
                                current_track_idx = 0;
                            } else if current_track_idx
                                >= playlists[current_playlist_idx].tracks.len()
                            {
                                current_track_idx = 0;
                            }

                            playlist_print(&playlists[current_playlist_idx]);
                        }
                    }

                    "" => {}
                    _ => println!("missing command"),
                }
            }
            thread::sleep(Duration::from_millis(50));
        }
    }
}

fn playlists_menu_print(playlists: &[scanner::Playlist]) {
    println!("\n--- Available Playlists ---");
    for (i, pl) in playlists.iter().enumerate() {
        println!("\t{}. {} ({} tracks)", i + 1, pl.name, pl.tracks.len());
    }
    println!("---------------------------");
}

fn playlist_print(playlist: &scanner::Playlist) {
    println!("\n--- Playlist: {} ---", playlist.name);
    for (i, track) in playlist.tracks.iter().enumerate() {
        let file_name = track
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown");
        println!("\t{}. {}", i + 1, file_name);
    }
    println!("----------------------------");
    println!("\tTotal tracks: {}", playlist.tracks.len());
}

fn help() {
    let help_print = [
        "Type 'cd <number>' to switch playlist",
        "pls or folders - list all available folders/playlists",
        "h or help - print all command",
        "q or quit - leave",
        "s or pause - stop play music",
        "p or play - play music",
        "n, next, f, forward - play next music",
        "b or back - play previous music",
        "-, low, l - decrease volume for 0.1",
        "ml, micro-low - decrease volume for 0.01",
        "+, high, u - increase volume for 0.1",
        "mh, micro-high - increase volume for 0.01",
        "ls, pl, list - print your playlist",
        "Audio directory: ~/SysPMF (or C:/Users/<User>/SysPMF)",
        "Place your audio files in ~/SysPMF",
    ];

    for element in help_print {
        println!("{element}");
    }
}
