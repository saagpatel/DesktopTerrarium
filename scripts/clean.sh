#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

# Backward-compatible wrapper for project-local heavy artifact cleanup.
"$repo_root/scripts/clean_heavy_artifacts.sh"
