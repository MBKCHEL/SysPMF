use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn play_playlist(playlist: &[PathBuf], player: &rodio::Player) {
    player.stop();
    for track_path in playlist {
        if let Ok(file) = File::open(track_path) {
            let reader = BufReader::new(file);
            if let Ok(source) = rodio::Decoder::new(reader) {
                player.append(source);
            }
        }
    }
    player.play();
}
