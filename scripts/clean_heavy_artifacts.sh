#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

bytes_before="$(du -sk . 2>/dev/null | awk '{print $1}')"
cleaned_any=0

# Remove local heavy build output and local audit scratch space.
for dir in target .codex_audit; do
  if [ -d "$dir" ]; then
    rm -rf "$dir"
    cleaned_any=1
  fi
done

bytes_after="$(du -sk . 2>/dev/null | awk '{print $1}')"
freed_kb=$((bytes_before - bytes_after))
if [ "$freed_kb" -lt 0 ]; then
  freed_kb=0
fi

if [ "$cleaned_any" -eq 1 ]; then
  echo "Heavy artifact cleanup complete. Freed approximately ${freed_kb} KB."
else
  echo "No heavy local artifacts found."
fi
