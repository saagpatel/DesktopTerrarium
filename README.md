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
- **Persistent state** — plant growth, weather, and activity stats saved every 5 minutes and on clean exit to the system data directory

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
cargo run --locked

# Release build
cargo build --locked --release
```

## Status

### Stack

| Layer | Technology |
|-------|------------|
| Language | Rust 2021 |
| Engine | Bevy 0.19 |
| Serialization | serde + serde_json |
| Persistence | JSON via `dirs` (platform data dir) |
| Idle detection | macOS CoreGraphics — CGEventSourceSecondsSinceLastEventType |

### Known risks

- Idle detection depends on macOS CoreGraphics. Other targets use an always-active fallback.
- Terrarium state persists to the platform data directory; do not reset personal state for a manual check.
- `.artifacts/` and `.perf-results/` contain tracked generated outputs and should not be swept into unrelated source commits.
- Plant art is currently placeholder art: all 15 plant stage images share identical image data.

### Next steps

- Replace the identical placeholder plant artwork with distinct species and stage art.

## Verification

Run these commands from the repository root (the directory containing
`Cargo.toml` and the committed `Cargo.lock`). Use the pinned Rust toolchain in
`rust-toolchain.toml`. Cargo may download dependencies on the first run;
`--locked` keeps the committed dependency resolution.

For the complete local check, run:

```bash
make check
```

For a focused activity-counter check that does not start the app or poll real
keyboard or mouse activity:

```bash
cargo test --locked --lib systems::behavior_tracker::tests
```

For the individual checks on macOS:

```bash
./scripts/check_local_artifacts.sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --no-deps -- -D warnings
cargo test --locked
cargo build --locked
```

Run `make audit` separately when `cargo-audit` is installed; it may update the
RustSec advisory database. The current tests exercise counter logic and
persistence in repository-local disposable test directories; they do not launch the GUI or
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

## License

MIT
