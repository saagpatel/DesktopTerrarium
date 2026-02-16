#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

tmp_base="${TMPDIR:-/tmp}"
session_dir="$(mktemp -d "${tmp_base%/}/desktop-terrarium-lean.XXXXXX")"
export CARGO_TARGET_DIR="${session_dir}/target"

cleanup() {
  if [ -n "${session_dir:-}" ] && [ -d "${session_dir}" ]; then
    rm -rf "${session_dir}"
  fi
}

trap cleanup EXIT INT TERM

echo "Lean dev mode: using temporary build artifacts in ${CARGO_TARGET_DIR}"
echo "Dependency downloads in \$HOME/.cargo are preserved for faster restarts."

cargo run "$@"
