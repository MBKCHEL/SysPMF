use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn play_track(playlist: &[PathBuf], index: usize, player: &rodio::Player)  {
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