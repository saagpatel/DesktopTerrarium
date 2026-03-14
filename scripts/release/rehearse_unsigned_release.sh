#!/usr/bin/env bash
set -euo pipefail

release_ref="${1:-v0.1.0-local-rehearsal-$(date +%Y%m%d-%H%M%S)}"

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

if ! command -v syft >/dev/null 2>&1; then
  echo "syft is required for unsigned rehearsal SBOM generation." >&2
  echo "Install with: brew install syft" >&2
  exit 1
fi

echo "Running release gate checks..."
./scripts/release/run_release_gate.sh

echo "Building architecture binaries for ${release_ref}..."
./scripts/release/macos_build_arch.sh aarch64-apple-darwin "$release_ref"
./scripts/release/macos_build_arch.sh x86_64-apple-darwin "$release_ref"

echo "Assembling universal package..."
./scripts/release/macos_assemble_universal.sh "$release_ref"
./scripts/release/macos_supply_chain.sh "$release_ref"

echo "Verifying generated checksums..."
(
  cd "dist/${release_ref}"
  shasum -a 256 -c SHA256SUMS
)

echo "Unsigned rehearsal complete: dist/${release_ref}"
