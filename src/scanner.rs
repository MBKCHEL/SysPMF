use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Playlist {
    pub name: String,
    pub tracks: Vec<PathBuf>,
}

pub fn scan_playlists(base_dir: &Path) -> Vec<Playlist> {
    let mut playlists = Vec::new();
    let root_tracks = collect_files_in_dir(base_dir);
    playlists.push(Playlist {
        name: "Root (SysPMF)".to_string(),
        tracks: root_tracks,
    });

    if let Ok(entries) = fs::read_dir(base_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let folder_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown")
                    .to_string();
                let tracks = collect_files_in_dir(&path);
                if !tracks.is_empty() {
                    playlists.push(Playlist {
                        name: folder_name,
                        tracks,
                    });
                }
            }
        }
    }

    playlists
}

fn collect_files_in_dir(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_lowercase())
                {
                    if ext == "mp3" || ext == "ogg" || ext == "wav" || ext == "flac" {
                        files.push(path);
                    }
                }
            }
        }
    }
    files.sort();
    files
}
