#!/usr/bin/env bash
set -euo pipefail

release_ref="${1:?release ref required}"

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

stage_root=".release-work/${release_ref}/stage"
dist_dir="dist/${release_ref}"
pkg_name="DesktopTerrarium-macos-universal"

if [[ ! -d "${stage_root}/${pkg_name}" ]]; then
  echo "Missing staged package directory: ${stage_root}/${pkg_name}" >&2
  exit 1
fi

if ! command -v syft >/dev/null 2>&1; then
  echo "syft is required to produce SPDX SBOM output." >&2
  echo "Install syft locally or use the release workflow setup step." >&2
  exit 1
fi

mkdir -p "$dist_dir"
tar -C "$stage_root" -czf "$dist_dir/${pkg_name}.tar.gz" "$pkg_name"
syft "dir:${stage_root}/${pkg_name}" -o spdx-json="$dist_dir/${pkg_name}.spdx.json"

(
  cd "$dist_dir"
  shasum -a 256 "${pkg_name}.tar.gz" "${pkg_name}.spdx.json" > SHA256SUMS
)
