# Desktop Terrarium

## Canonical commands

The repository defines verification commands in CI at `.github/workflows/ci.yml`:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --no-deps`
- `cargo test`
- `cargo build`
- `cargo check` (Linux compile check job)

This is a Rust Cargo project (`Cargo.toml`), so local run uses Cargo defaults:

- Normal dev run: `cargo run`

## Development modes

### Normal dev

Use normal Cargo behavior for fastest iterative builds:

```bash
cargo run
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
