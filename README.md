# Desktop Terrarium

## Canonical commands

The canonical local verification list lives in `.codex/verify.commands`.
CI mirrors the same contract through `.github/workflows/ci.yml`.

The repo-defined checks are:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --no-deps -- -D warnings`
- `cargo nextest run --workspace --all-targets --profile release --locked`
- `cargo test --workspace --doc --locked`
- `cargo build --workspace --all-targets --locked`
- `cargo deny check --config deny.toml advisories bans licenses sources`
- `cargo audit --deny warnings`
- `./scripts/perf/native-smoke.sh`
- `./scripts/perf/compare-native-metrics.sh .perf-baselines/native.json .perf-results/native.json`

This is a Rust Cargo project (`Cargo.toml`). Always run cargo commands through:

- `./scripts/env/with_deterministic_env.sh`

This prevents platform-specific path issues and keeps local and CI behavior aligned.

Example:

```bash
./scripts/env/with_deterministic_env.sh cargo run
```

To run the full canonical list deterministically:

```bash
./.codex/scripts/run_verify_commands.sh
```

## Development modes

### Normal dev

Use normal Cargo behavior for fastest iterative builds:

```bash
./scripts/env/with_deterministic_env.sh cargo run
```

Disk tradeoff:
- Fastest restarts once `target/` is warm.
- Uses persistent local build artifacts in `target/`.

### Lean dev (low disk)

Use temporary build output that is cleaned automatically when the app exits:

```bash
./scripts/run_lean_dev.sh
```

Disk tradeoff:
- Keeps repo disk growth low by avoiding persistent `target/`.
- Slower startup because rebuild work is repeated each lean session.
- Preserves `$HOME/.cargo` dependency cache for reasonable download/build speed.

## Smoke testing

Exact smoke instructions live in `docs/operations/local-smoke-walkthrough.md`.
Visual art direction and import pipeline docs live in:

- `docs/operations/visual-style-bible.md`
- `docs/operations/asset-production-brief.md`
- `docs/operations/asset-import-contract.md`

Fast automated smoke against an isolated temp state directory:

```bash
./scripts/smoke/run_local_smoke.sh
```

Interactive smoke uses the in-app overlay and debug controls:

- `F1`: toggle overlay
- `F2`: force time-of-day phase
- `F3`: force activity mode
- `F4`: force weather
- `F5`: cycle time scale
- `F6`: cycle plant growth multiplier
- `F7`: advance plant stages
- `F8`: spawn beetle
- `F9`: spawn butterfly
- `F10`: save immediately

Use `Fn` with function keys if macOS media-key mode is enabled.

## Cleanup commands

Targeted cleanup (heavy local artifacts only):

```bash
./scripts/clean_heavy_artifacts.sh
```

Full local cleanup (all reproducible caches, including global Cargo download/source cache):

```bash
./scripts/clean_full_local.sh
```

Compatibility wrapper:

```bash
./scripts/clean.sh
```

## Notes

- Cleanup scripts are non-destructive to source files and Git history.
- Full cleanup intentionally removes `$HOME/.cargo/registry` and `$HOME/.cargo/git`, which increases next build/download time.

## Execution Profiles

Execution profile definitions live in `docs/operations/execution-profiles.md`.

## Feature Modes

Weather behavior can be switched at runtime using `TERRARIUM_WEATHER_MODE`:

- `full` (default): transition + particles.
- `reduced`: transitions enabled, particles disabled.
- `safe_disable`: weather forced to clear.
- `static_baseline`: weather forced to clear for baseline/recovery runs.

Example:

```bash
TERRARIUM_WEATHER_MODE=reduced ./scripts/env/with_deterministic_env.sh cargo run
```

To isolate smoke or release rehearsals from your normal save data, override the state directory:

```bash
TERRARIUM_STATE_DIR="$(mktemp -d)" ./scripts/env/with_deterministic_env.sh cargo run
```

## Quality And Perf Workflows

- `CI`: format, clippy, nextest, doctests, build, supply-chain checks.
- `native-perf`: Rust-native performance baseline enforcement.
- `codeql-rust`: static analysis for Rust.
- `legacy-web-perf-*`: manual legacy checks (not ship-blocking for this desktop Rust app).

The file `tests/perf/api.k6.js` is a legacy placeholder and is not part of the desktop release gate.

## Release

Release architecture and checklists:

- `docs/release/macos-binary-release-architecture.md`
- `docs/release/checklists/macos-staged-release-checklist.md`
- `docs/release/current-readiness.md`

Main release workflow:

- `.github/workflows/release-macos.yml`

Release preflight (no credentials required):

```bash
./scripts/release/run_release_gate.sh
```

Unsigned local rehearsal (build + package + SBOM + checksums):

```bash
./scripts/release/rehearse_unsigned_release.sh
```

Packaged binary sanity smoke after rehearsal:

```bash
./scripts/release/smoke_packaged_app.sh <tag>
```

Notes:
- `release-macos` is manual (`workflow_dispatch`) and resolves the exact tag provided.
- GitHub release publishing is enabled only when `sign=true`.
- `docs/release/current-readiness.md` is the current branch-level truth for what has been validated locally versus what still requires production credentials.
