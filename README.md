# 📻 Rust Radio TUI

A beautiful terminal radio player with dynamic station search powered by [radio-browser.info](https://radio-browser.info).

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## ✨ Features

- 🔍 **Dynamic Search** - Search stations by name, genre, or country (e.g., `ua:rock`, `jazz`, `us:news`)
- 🎨 **Beautiful TUI** - Modern terminal interface built with `ratatui`
- 🔊 **System Volume Control** - Integrated with `wpctl` (PipeWire)
- ⚡ **Fast & Responsive** - No UI lag, instant feedback
- 📦 **Standalone Binary** - Works on any Linux system
- 🎵 **High Quality Streams** - Only working, high-bitrate stations

## 📸 Screenshots

```
┌─────────────────────────────────────┐
│ 📻 Rust Radio TUI                   │
├─────────────────────────────────────┤
│ ▶ Radio Paradise | US | Rock        │
│   SomaFM Groove Salad | US | Ambient│
│   BBC Radio 1 | GB | Pop            │
│   Radio Swiss Jazz | CH | Jazz      │
├─────────────────────────────────────┤
│ 🔍 Search: ua:rock                  │
│ Vol: 65% | q:quit | /:search        │
└─────────────────────────────────────┘
```

## 🚀 Installation

### Option 1: Download Pre-built Binary

```bash
# Download the latest release
wget https://github.com/Yuralitv1982/rust-radio-tui/releases/latest/download/rust-radio
chmod +x rust-radio
sudo mv rust-radio /usr/local/bin/r-radio
r-radio
```

### Option 2: Build from Source

```bash
git clone https://github.com/Yuralitv1982/rust-radio-tui.git
cd rust-radio-tui
cargo build --release
./target/release/rust-radio
```

### Option 3: Install via Cargo

```bash
cargo install --git https://github.com/Yuralitv1982/rust-radio-tui
```

## 🎮 Usage

### Basic Controls

| Key | Action |
|-----|--------|
| `↑` / `↓` | Navigate up/down |
| `Enter` | Play selected station |
| `Space` | Stop playback |
| `+` / `-` | Volume up/down (+5%/-5%) |
| `q` | Quit application |

### Search & Discovery

| Key | Action |
|-----|--------|
| `/` | Enter search mode |
| `s` | Enter search mode (alternative) |
| `Esc` | Clear search / exit search mode |
| `Ctrl+R` | Refresh station list (load global top) |

### Search Examples

Type in search mode (`/`):
- `jazz` - Search for jazz stations worldwide
- `ua:rock` - Search for rock stations in Ukraine 🇺🇦
- `us:news` - Search for news stations in USA 🇺🇸
- `de:classical` - Search for classical stations in Germany 🇩🇪
- `bbc` - Search stations with "bbc" in name

## 🔧 Requirements

- **Linux** (tested on Fedora, Ubuntu, Debian)
- **mpv** - Media player backend
- **wpctl** - PipeWire audio control (or PulseAudio)

### Install Dependencies

**Fedora/RHEL:**
```bash
sudo dnf install mpv wireplumber
```

**Debian/Ubuntu:**
```bash
sudo apt install mpv pipewire
```

## 🛠️ Building

### Development Build
```bash
cargo build
```

### Optimized Release Build
```bash
cargo build --release
```

### Static Binary (Musl)
```bash
sudo dnf install musl-gcc
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

## 📁 Configuration

- `~/.config/rust-radio/` - Configuration directory (future feature)
- `~/.local/share/rust-radio/` - Data directory (future feature)

## 🏗️ Technologies

- **Language:** Rust 🦀
- **TUI Framework:** ratatui
- **Async Runtime:** tokio
- **HTTP Client:** reqwest with rustls
- **Audio Backend:** mpv + wpctl (PipeWire)
- **API:** radio-browser.info

## 📝 Roadmap

- [ ] Playlist management (save/load favorite stations)
- [ ] Auto-play last station on startup
- [ ] Visual audio indicator / spectrum visualizer
- [ ] Daemon mode (run in background)
- [ ] Configuration file support

## 📄 License

MIT License - see LICENSE file for details.

## 🙏 Acknowledgments

- Thanks to radio-browser.info for providing free radio station API
- Inspired by terminal music players like ncmpcpp and cmus
- Built with ❤️ using Rust

## 📬 Contact

- **GitHub:** [@Yuralitv1982](https://github.com/Yuralitv1982)
- **Project Link:** https://github.com/Yuralitv1982/rust-radio-tui

---
<div align="center">
<b>Made with 🦀 and ☕</b><br>
⭐ Star this repo if you like it!
</div>
