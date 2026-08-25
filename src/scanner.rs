use std::fs;
use std::path::{Path, PathBuf};

pub fn find_audio_files<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
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

