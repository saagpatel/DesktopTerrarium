# Desktop Terrarium

[![Rust](https://img.shields.io/badge/rust-%23dea584?style=flat-square&logo=rust)](#) [![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](#)

> Stay focused long enough and a butterfly appears. Your terrarium knows if you've been slacking.

Desktop Terrarium is a native desktop app (800×600, resizable) that renders a layered 2D terrarium scene using the Bevy game engine. The scene evolves based on your real keyboard activity and an internal clock — plants grow through life stages, weather transitions between four states with particle effects, and critters visit when you stay focused.

## Features

- **Activity-driven growth** — three plant species (Fern, Moss, Succulent) advance through growth stages; idle time detected via macOS CoreGraphics
- **Dynamic weather** — Clear, Fog, Rain, and Wind states cycle over 5-minute phases with 30-second particle-effect transitions
- **Critter visits** — butterfly appears after 30 consecutive focus minutes; beetle visits randomly during active time
- **Time-of-day cycle** — morning, day, evening, night phases crossfade layered sprites
- **Parallax layers** — scene layers move at different depths as the window is interacted with
- **Persistent state** — plant growth, weather, and activity stats saved every 5 minutes to the system data directory

## Quick Start

### Prerequisites
- Rust stable toolchain
- macOS (idle detection uses CoreGraphics)

### Installation
```bash
git clone https://github.com/saagpatel/DesktopTerrarium
cd DesktopTerrarium
```

### Usage
```bash
# Run
cargo run

# Release build
cargo build --release
```

## Tech Stack

| Layer | Technology |
|-------|------------|
| Language | Rust 2021 |
| Engine | Bevy 0.15 |
| Serialization | serde + serde_json |
| Persistence | JSON via `dirs` (platform data dir) |
| Idle detection | macOS CoreGraphics — CGEventSourceSecondsSinceLastEventType |

## License

MIT
