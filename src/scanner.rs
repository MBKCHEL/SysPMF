use std::fs;
use std::path::{Path, PathBuf};

const AUDIO_EXT: &[&str] = &["mp3", "ogg", "wav", "flac", "opus", "m4a"];

#[derive(Clone, Debug)]
pub struct Playlist {
    pub name: String,
    pub tracks: Vec<PathBuf>,
}

pub fn scan_playlists(base_dir: &Path) -> Vec<Playlist> {
    let mut playlists = Vec::new();

    playlists.push(Playlist {
        name: "Root (SysPMF)".to_string(),
        tracks: collect_files_in_dir(base_dir),
    });
    playlists.extend(collect_m3u_in_dir(base_dir));

    if let Ok(entries) = fs::read_dir(base_dir) {
        let mut dirs: Vec<PathBuf> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();
        dirs.sort();

        for path in dirs {
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
            playlists.extend(collect_m3u_in_dir(&path));
        }
    }

    playlists
}

fn is_audio(path: &Path) -> bool {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|s| AUDIO_EXT.contains(&s.to_lowercase().as_str()))
        .unwrap_or(false)
}

fn collect_files_in_dir(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && is_audio(&path) {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

fn collect_m3u_in_dir(dir: &Path) -> Vec<Playlist> {
    let mut result = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return result;
    };

    let mut files: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.is_file()
                && p.extension()
                .and_then(|s| s.to_str())
                .map(|s| matches!(s.to_lowercase().as_str(), "m3u" | "m3u8"))
                .unwrap_or(false)
        })
        .collect();
    files.sort();

    for m3u in files {
        let tracks = parse_m3u(&m3u);
        if tracks.is_empty() {
            continue;
        }
        let stem = m3u
            .file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("playlist");
        result.push(Playlist {
            name: format!("[m3u] {stem}"),
            tracks,
        });
    }
    result
}

fn parse_m3u(m3u_path: &Path) -> Vec<PathBuf> {
    let base = m3u_path.parent().unwrap_or_else(|| Path::new("."));

    let Ok(bytes) = fs::read(m3u_path) else {
        return Vec::new();
    };
    let text = String::from_utf8_lossy(&bytes);
    let text = text.trim_start_matches('\u{feff}');

    let mut tracks = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.contains("://") && !line.starts_with("file://") {
            continue;
        }

        let raw = line.strip_prefix("file://").unwrap_or(line).replace('\\', "/");
        let p = PathBuf::from(raw);
        let full = if p.is_absolute() { p } else { base.join(p) };

        if full.is_file() && is_audio(&full) {
            tracks.push(full);
        }
    }
    tracks
}