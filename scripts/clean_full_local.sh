#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

repo_before_kb="$(du -sk . 2>/dev/null | awk '{print $1}')"
cargo_registry_before_kb=0
cargo_git_before_kb=0
[ -d "$HOME/.cargo/registry" ] && cargo_registry_before_kb="$(du -sk "$HOME/.cargo/registry" 2>/dev/null | awk '{print $1}')"
[ -d "$HOME/.cargo/git" ] && cargo_git_before_kb="$(du -sk "$HOME/.cargo/git" 2>/dev/null | awk '{print $1}')"

# First remove repo-local heavy build artifacts.
"$repo_root/scripts/clean_heavy_artifacts.sh"

# Then remove reproducible global Cargo download/source caches.
if [ -d "$HOME/.cargo/registry" ]; then
  rm -rf "$HOME/.cargo/registry"
fi

if [ -d "$HOME/.cargo/git" ]; then
  rm -rf "$HOME/.cargo/git"
fi

repo_after_kb="$(du -sk . 2>/dev/null | awk '{print $1}')"
cargo_registry_after_kb=0
cargo_git_after_kb=0
[ -d "$HOME/.cargo/registry" ] && cargo_registry_after_kb="$(du -sk "$HOME/.cargo/registry" 2>/dev/null | awk '{print $1}')"
[ -d "$HOME/.cargo/git" ] && cargo_git_after_kb="$(du -sk "$HOME/.cargo/git" 2>/dev/null | awk '{print $1}')"

repo_freed_kb=$((repo_before_kb - repo_after_kb))
cargo_registry_freed_kb=$((cargo_registry_before_kb - cargo_registry_after_kb))
cargo_git_freed_kb=$((cargo_git_before_kb - cargo_git_after_kb))
total_freed_kb=$((repo_freed_kb + cargo_registry_freed_kb + cargo_git_freed_kb))

if [ "$repo_freed_kb" -lt 0 ]; then repo_freed_kb=0; fi
if [ "$cargo_registry_freed_kb" -lt 0 ]; then cargo_registry_freed_kb=0; fi
if [ "$cargo_git_freed_kb" -lt 0 ]; then cargo_git_freed_kb=0; fi
if [ "$total_freed_kb" -lt 0 ]; then total_freed_kb=0; fi

echo "Full local cleanup complete."
echo "Freed approximately ${repo_freed_kb} KB in repo-local artifacts."
echo "Freed approximately ${cargo_registry_freed_kb} KB in \$HOME/.cargo/registry."
echo "Freed approximately ${cargo_git_freed_kb} KB in \$HOME/.cargo/git."
echo "Total freed approximately ${total_freed_kb} KB."
