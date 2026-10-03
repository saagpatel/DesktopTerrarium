# Desktop Terrarium

[![Rust](https://img.shields.io/badge/rust-%23dea584?style=flat-square&logo=rust)](#) [![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](#)

> Stay focused long enough and a butterfly appears. Your terrarium knows if you've been slacking.

Desktop Terrarium is a native desktop app (800×600, resizable) that renders a layered 2D terrarium scene using the Bevy game engine. The scene evolves based on your real keyboard and mouse activity and an internal clock — plants grow through life stages, weather transitions between four states with particle effects, and critters visit when you stay focused.

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

## Verification

Run these commands from the repository root (the directory containing
`Cargo.toml` and the committed `Cargo.lock`). Use Rust stable with the `rustfmt`
and `clippy` components installed. Cargo may download dependencies on the first
run; `--locked` keeps the committed dependency resolution.

For a focused activity-counter check that does not start the app or poll real
keyboard or mouse activity:

```bash
cargo test --locked --lib systems::behavior_tracker::tests
```

For broader local verification on macOS:

```bash
./scripts/check_local_artifacts.sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --no-deps
cargo test --locked
cargo build --locked
cargo audit
```

`cargo audit` requires the separately installed `cargo-audit` tool and may update
its advisory database; install it with `cargo install cargo-audit`. The current tests exercise counter logic and
persistence in disposable temporary directories; they do not launch the GUI or
load the app's personal state. Build outputs go into `target/` by default.

macOS is the supported idle-detection platform. Non-macOS builds use an
always-active fallback, so compiling elsewhere does not prove activity tracking.

For changes to rendering, interaction, activity detection or app persistence,
also check the native app on macOS with `cargo run --locked`: confirm the scene,
resize behavior and the affected interaction. This launches a real window,
reads system idle time and loads/saves state in the platform data directory under
`com.desktopterrarium.app`. Use a separate disposable OS account for an isolated
manual check; do not delete or reset existing personal state. No browser check
applies to this native app, and a documentation-only change does not require
launching it.

## Tech Stack

| Layer | Technology |
|-------|------------|
| Language | Rust 2021 |
| Engine | Bevy 0.19 |
| Serialization | serde + serde_json |
| Persistence | JSON via `dirs` (platform data dir) |
| Idle detection | macOS CoreGraphics — CGEventSourceSecondsSinceLastEventType |

## License

MIT
