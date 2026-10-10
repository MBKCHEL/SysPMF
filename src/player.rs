use std::path::Path;
use crate::dekoder;

pub fn play_track<P: AsRef<Path>>(track_path: P, player: &rodio::Player) -> bool {
    player.stop();
    match dekoder::AnySource::open(track_path) {
        Ok(source) => {
            player.append(source);
            player.play();
            true
        }
        Err(e) => {
            eprintln!("Не удалось открыть трек: {e}");
            false
        }
    }
}
