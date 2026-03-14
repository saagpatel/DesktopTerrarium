# Definition Of Done

## Build and Test

- `./scripts/env/with_deterministic_env.sh cargo fmt --all -- --check`
- `./scripts/env/with_deterministic_env.sh cargo clippy --workspace --all-targets --no-deps -- -D warnings`
- `./scripts/env/with_deterministic_env.sh cargo nextest run --workspace --all-targets --profile release --locked`
- `./scripts/env/with_deterministic_env.sh cargo test --workspace --doc --locked`
- `./scripts/env/with_deterministic_env.sh cargo build --workspace --all-targets --locked`

## Security and Supply Chain

- `cargo deny` checks pass against `deny.toml`.
- `cargo audit --deny warnings` passes.
- CodeQL workflow is green.
- Release artifacts include checksums, attestation, and SBOM output.

## Release Readiness

- Native perf workflow passes against `.perf-baselines/native.json`.
- Release workflow can produce universal macOS archive from both architectures.
- Local smoke script and packaged-app smoke both pass on macOS.
- Rollback and staged rollout checklists are complete and usable.
