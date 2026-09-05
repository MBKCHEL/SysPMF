# SysPMF (SysPrint Music Player)

A fast, minimalist CLI-based music player written in Rust.

## Features

- **Lightweight & Fast:** Zero heavy GUI overhead, runs directly in your terminal.
- **Auto Directory Init:** Automatically creates `~/SysPMF` (or `C:\Users\<User>\SysPMF` on Windows) on first launch.
- **Simple Controls:** Intuitive single-key commands for playback and volume.

## Controls

* **`p` / `play`** - Play music
* **`s` / `pause`** - Pause playback
* **`+` / `u` / `high`** - Increase volume for 0.1
* **`-` / `l` / `low`** - Decrease volume for 0.1
* **`h` / `help`** - Show help menu
* **`q` / `quit`** - Exit
* **`n` / `f` / `forward` / `next`** - next track
* **`b` / `back`** - play previous music
* **`ml` / `micro-low`** - Decrease volume for 0.01
* **`mh` / `micro-high`** - Increase volume for 0.01
  
## Installation (Linux)
### Fast Install (Pre-compiled Binary)

1. Download the compiled binary from Releases:
```bash
https://github.com/MBKCHEL/SysPMF/releases/tag/1.0.0
```
Make it executable:

``` bash
chmod +x ~/Downloads/syspmf-linux

```
### Move it to your local path to run it from anywhere:
``` bash
sudo mv ~/Downloads/syspmf-linux /usr/local/bin/syspmf
```

### Run the player:
``` bash
syspmf
```

## Uninstallation

To remove SysPMF from your system:

```bash
sudo rm /usr/local/bin/syspmf
```
