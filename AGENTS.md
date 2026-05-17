<!-- portfolio-context:start -->
# Portfolio Context

## What This Project Is

Desktop Terrarium is a native Bevy desktop app that grows a layered 2D terrarium based on real keyboard activity and time. Plants advance through growth stages, weather cycles through particle-rich states, critters appear after focus streaks, and state persists locally.

## Current State

The repo is active local desktop/game work. Existing untracked folders are generated local artifacts, so this recovery pass should only add the new context file.

## Stack

| Layer | Technology |
|-------|------------|
| Language | Rust 2021 |
| Engine | Bevy 0.15 |
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

## Known Risks

- Idle detection depends on macOS CoreGraphics, so cross-platform claims need explicit verification.
- Terrarium state persists to the platform data directory; avoid destructive state resets without operator approval.
- Generated `.artifacts` and `.perf-results` folders are local outputs and should not be swept into source commits.
- Keep Bevy performance and 800x600/resizable window behavior intact when changing rendering.

## Next Recommended Move

Add only the context file for this recovery pass, then keep future work focused on activity-driven growth, local persistence, and Bevy rendering stability.

<!-- portfolio-context:end -->
