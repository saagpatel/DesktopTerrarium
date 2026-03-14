#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

./scripts/check_local_artifacts.sh
./scripts/env/with_deterministic_env.sh cargo fmt --all -- --check
./scripts/env/with_deterministic_env.sh cargo clippy --workspace --all-targets --no-deps -- -D warnings
./scripts/env/with_deterministic_env.sh cargo nextest run --workspace --all-targets --profile release --locked
./scripts/env/with_deterministic_env.sh cargo test --workspace --doc --locked
./scripts/env/with_deterministic_env.sh cargo deny check --config deny.toml advisories bans licenses sources
./scripts/env/with_deterministic_env.sh cargo audit --deny warnings
./scripts/perf/native-smoke.sh
./scripts/perf/compare-native-metrics.sh .perf-baselines/native.json .perf-results/native.json

echo "Release gate checks passed."
