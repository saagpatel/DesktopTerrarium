[![Rust](https://img.shields.io/badge/rust-2021--edition-orange?logo=rust)](https://www.rust-lang.org/)
[![Bevy](https://img.shields.io/badge/bevy-0.15-blue?logo=bevyengine)](https://bevyengine.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-green)](LICENSE)

# Desktop Terrarium

A living desktop terrarium that responds to how you work. Plants grow, weather shifts, and small critters visit — all driven by your real-world activity at the keyboard. The longer you stay focused, the richer the scene becomes.

---

## What it does

Desktop Terrarium is a native desktop app (800×600, resizable) that renders a layered 2D terrarium scene using the Bevy game engine. The scene is not static — it evolves based on system behavior signals and an internal clock:

- **Time of day** — morning, day, evening, and night phases cycle through the scene, crossfading layered sprites (glass container, soil, backdrop) to match the current phase.
- **Plant growth** — three plant species (Fern, Moss, Succulent) advance through growth stages over time, with progress persisted between sessions.
- **Weather system** — the scene transitions between Clear, Fog, Rain, and Wind states over 5-minute phases. Each weather type spawns its own particle effects with smooth 30-second fades between states.
- **Critter visits** — a butterfly appears after 30 consecutive minutes of focus; a beetle has a chance to visit each minute while the system is active. Critters follow randomized curved paths across the scene.
- **Activity tracking** — idle time is polled via the macOS CoreGraphics API. After 2 minutes of inactivity the focus streak resets; total active time and the longest focus streak are recorded.
- **Parallax layers** — scene layers move at different depths as the window is interacted with.
- **Persistent state** — plant growth, weather, time-of-day phase, and activity stats are saved every 5 minutes to the system data directory and restored on next launch.

---

## Tech stack

| Layer | Technology |
|---|---|
| Language | Rust 2021 |
| Engine | Bevy 0.15 |
| Serialization | serde + serde_json |
| Persistence | JSON via `dirs` (platform data dir) |
| Date/time | chrono |
| Idle detection | macOS CoreGraphics (`CGEventSourceSecondsSinceLastEventType`) |
| Windowing | Wayland feature flag enabled |

---

## Prerequisites

- Rust toolchain (stable, 1.75+) — install via [rustup](https://rustup.rs/)
- macOS (primary target; the idle-detection system call is macOS-only; other platforms compile with a stub that assumes the user is always active)
- GPU driver supporting Metal (macOS default)

---

## Getting started

```bash
git clone https://github.com/saagpatel/DesktopTerrarium.git
cd DesktopTerrarium
cargo run
```

For a faster development build with optimized dependencies:

```bash
cargo run --profile dev   # uses opt-level 1 for the app, opt-level 3 for deps
```

To run the test suite:

```bash
cargo test
```

---

## Project structure

```
src/
  main.rs               # App entry point, window config
  lib.rs                # Module re-exports
  plugins/              # TerrariumPlugin (wires all systems + events)
  components/           # ECS components: Plant, Critter, SceneLayer, etc.
  resources/            # ECS resources: TimeOfDay, WeatherState, BehaviorSignals
  systems/              # All game systems (one file per concern)
    setup.rs            # Scene spawn (camera, layers, plants)
    time_of_day.rs      # Phase cycle + sprite crossfade
    weather_transition.rs
    rain / fog / wind   # Particle spawn + update per weather type
    plant_growth.rs     # Growth stage advancement
    critter_spawner.rs  # Behavior-gated critter arrival
    critter_movement.rs # Path following
    behavior_tracker.rs # Idle polling + streak counters
    parallax.rs         # Depth-based layer movement
    persistence.rs      # Save/load state.json
    adaptive_framerate.rs
    debug_ui.rs         # Debug input handling
  events/               # PlantStageChanged, CritterArrived/Departed, WeatherChanged
  errors.rs             # Typed error hierarchy (TerrariumError)
assets/
  layers/               # Backdrop, glass container, soil sprites (per time phase)
  plants/               # Per-species, per-stage sprites
  critters/             # Butterfly, beetle sprites
```

---

## Screenshot

> _No screenshot available yet — run the app to see the terrarium live._

---

## License

MIT — see [LICENSE](LICENSE).
