#!/usr/bin/env bash
set -euo pipefail

release_ref="${1:?release ref required}"

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

arm_bin=".release-work/${release_ref}/downloads/bin-aarch64-apple-darwin/desktop_terrarium"
x86_bin=".release-work/${release_ref}/downloads/bin-x86_64-apple-darwin/desktop_terrarium"
if [[ ! -f "$arm_bin" ]]; then
  arm_bin=".release-work/${release_ref}/aarch64-apple-darwin/desktop_terrarium"
fi
if [[ ! -f "$x86_bin" ]]; then
  x86_bin=".release-work/${release_ref}/x86_64-apple-darwin/desktop_terrarium"
fi
stage_dir=".release-work/${release_ref}/stage/DesktopTerrarium-macos-universal"

if [[ ! -f "$arm_bin" || ! -f "$x86_bin" ]]; then
  echo "Missing architecture artifacts for universal binary assembly." >&2
  exit 1
fi

mkdir -p "$stage_dir"
lipo -create -output "$stage_dir/desktop_terrarium" "$arm_bin" "$x86_bin"
lipo -info "$stage_dir/desktop_terrarium"

cp -R assets "$stage_dir/assets"
