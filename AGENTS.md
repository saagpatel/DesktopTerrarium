<!-- portfolio-context:start -->
# Portfolio Context

## What This Project Is

Desktop Terrarium is a native Bevy desktop app that grows a layered 2D terrarium based on real keyboard and mouse activity and time. Plants advance through growth stages, weather cycles through particle-rich states, butterflies appear after focus streaks, beetles visit randomly during active time, and state persists locally.

## Current State

The repo is active local desktop/game work. Generated outputs `.artifacts/readiness-summary.md` and `.perf-results/native.json` are already tracked.

## Stack

| Layer | Technology |
|-------|------------|
| Language | Rust 2021 |
| Engine | Bevy 0.19 |
| Serialization | serde + serde_json |
| Persistence | JSON via `dirs` (platform data dir) |
| Idle detection | macOS CoreGraphics — CGEventSourceSecondsSinceLastEventType |

## How To Run

```bash
# Run
cargo run

# Release build
cargo build --release
```

## Verification

Follow the authoritative [README verification instructions](README.md#verification).
Use the focused activity-counter unit tests before any interactive app check.

## Known Risks

- Idle detection depends on macOS CoreGraphics, so cross-platform claims need explicit verification.
- Terrarium state persists to the platform data directory; avoid destructive state resets without operator approval.
- Generated `.artifacts` and `.perf-results` folders contain tracked outputs; additional local outputs should not be swept into source commits.
- Keep Bevy performance and 800x600/resizable window behavior intact when changing rendering.

## Next Recommended Move

Keep future work focused on activity-driven growth, local persistence, and Bevy rendering stability.

<!-- portfolio-context:end -->
