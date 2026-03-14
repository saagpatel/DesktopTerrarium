#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

if [[ -f .codex/actions/_artifact_env.sh ]]; then
  # shellcheck source=/dev/null
  source .codex/actions/_artifact_env.sh
fi

if [[ -z "${CARGO_TARGET_DIR:-}" ]]; then
  if [[ "$(uname -s)" == "Darwin" ]]; then
    cache_root="${CODEX_CACHE_ROOT:-$HOME/Library/Caches/Codex}"
  else
    cache_root="${CODEX_CACHE_ROOT:-${XDG_CACHE_HOME:-$HOME/.cache}/codex}"
  fi

  if command -v shasum >/dev/null 2>&1; then
    repo_hash="$(printf '%s' "$repo_root" | shasum -a 256 | awk '{print substr($1,1,12)}')"
  else
    repo_hash="$(printf '%s' "$repo_root" | md5sum | awk '{print substr($1,1,12)}')"
  fi

  export CARGO_TARGET_DIR="$cache_root/build/rust/$repo_hash"
fi

mkdir -p "$CARGO_TARGET_DIR"

if [[ "$#" -eq 0 ]]; then
  echo "CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
  exit 0
fi

exec "$@"
