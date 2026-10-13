# Purpose

Desktop Terrarium is a native Bevy 0.19 app that grows a layered terrarium from keyboard and mouse activity and elapsed time. State is stored locally as JSON.

# Layout

- `src/components`, `src/resources`, `src/events`, and `src/systems` contain Bevy application code.
- `assets/` contains runtime sprites and particles.
- `scripts/check_local_artifacts.sh` guards tracked local build artifacts.
- `Cargo.toml` and `Cargo.lock` define the Rust package and locked dependency graph.

# Setup

Install the pinned Rust toolchain with `rustup` by running `rustup toolchain install 1.99.0 --component clippy --component rustfmt`. Then run the app with `cargo run --locked`.

# Checks

- `make check` runs the non-mutating local artifact, formatting, lint, test, and build checks.
- `make audit` runs `cargo audit`; it requires `cargo-audit` and an available RustSec advisory database.
- Focused activity tests: `cargo test --locked --lib systems::behavior_tracker::tests`.

# Boundaries

- Idle detection uses macOS CoreGraphics. Other targets use an always-active fallback and do not establish macOS activity behavior.
- Persistence writes to the platform data directory. Never delete or reset user save data as part of routine development.
- Preserve the tracked generated outputs `.artifacts/` and `.perf-results/`. Do not sweep additional local outputs into source changes.
- Keep Bevy performance and the 800x600 resizable window behavior intact when changing rendering.
- GUI launch, screenshots, publishing, and release work are outside routine source checks.

# Delivery

Keep changes focused on activity-driven growth, local persistence, and Bevy rendering stability. Commit using a conventional commit header and do not include real local paths, personal names, email addresses, or AI co-author trailers.
