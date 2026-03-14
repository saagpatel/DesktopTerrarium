#!/usr/bin/env bash
set -euo pipefail

release_ref="${1:?usage: ./scripts/release/smoke_packaged_app.sh <tag>}"

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

archive="dist/${release_ref}/DesktopTerrarium-macos-universal.tar.gz"
checksum_file="dist/${release_ref}/SHA256SUMS"

if [[ ! -f "$archive" ]]; then
  echo "Missing packaged archive: $archive" >&2
  exit 1
fi

if [[ ! -f "$checksum_file" ]]; then
  echo "Missing checksum file: $checksum_file" >&2
  exit 1
fi

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/desktop-terrarium-packaged-smoke.XXXXXX")"
state_dir="$tmp_dir/state"
mkdir -p "$state_dir"

tar -xzf "$archive" -C "$tmp_dir"

binary="$tmp_dir/DesktopTerrarium-macos-universal/desktop_terrarium"
if [[ ! -x "$binary" ]]; then
  echo "Missing packaged binary after extraction: $binary" >&2
  exit 1
fi

(
  cd "dist/${release_ref}"
  shasum -a 256 -c SHA256SUMS
)

echo "Launching packaged binary smoke run..."
TERRARIUM_STATE_DIR="$state_dir" TERRARIUM_SMOKE_SCRIPT=1 "$binary"

state_file="$state_dir/state.json"
if [[ ! -f "$state_file" ]]; then
  echo "Packaged smoke run did not persist a state file: $state_file" >&2
  exit 1
fi

echo "Packaged smoke passed: $archive"
