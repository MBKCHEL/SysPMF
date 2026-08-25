use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn play_track<P: AsRef<Path>>(track_path: P, player: &rodio::Player) -> bool {
    player.stop();
    if let Ok(file) = File::open(track_path) {
        let reader = BufReader::new(file);
        if let Ok(source) = rodio::Decoder::new(reader) {
            player.append(source);
            player.play();
            return true;
        }
    }
    false
}
