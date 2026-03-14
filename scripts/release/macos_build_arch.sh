#!/usr/bin/env bash
set -euo pipefail

target="${1:?target triple required}"
release_ref="${2:?release ref required}"

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

if [[ -z "${CARGO_TARGET_DIR:-}" ]]; then
  export CARGO_TARGET_DIR="$(./scripts/env/with_deterministic_env.sh | sed -n 's/^CARGO_TARGET_DIR=//p')"
fi
mkdir -p "$CARGO_TARGET_DIR"

rustup target add "$target"
./scripts/env/with_deterministic_env.sh cargo build --workspace --release --locked --target "$target"

out_dir=".release-work/${release_ref}/${target}"
mkdir -p "$out_dir"

bin_path="${CARGO_TARGET_DIR}/${target}/release/desktop_terrarium"
if [[ ! -f "$bin_path" ]]; then
  bin_path="target/${target}/release/desktop_terrarium"
fi
cp "$bin_path" "$out_dir/desktop_terrarium"
chmod +x "$out_dir/desktop_terrarium"
