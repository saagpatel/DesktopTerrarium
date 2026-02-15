#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

bytes_before="$(du -sk . 2>/dev/null | awk '{print $1}')"

cleaned_any=0

# Remove macOS finder metadata files that can appear in project trees.
while IFS= read -r ds_store; do
  if find "$ds_store" -maxdepth 1 -name '.DS_Store' -print -quit | grep -q .; then
    find "$ds_store" -maxdepth 1 -name '.DS_Store' -delete
    cleaned_any=1
  fi
done < <(printf '%s\n' "$repo_root" "$(dirname "$repo_root")")

# Remove generated Rust build output.
if [ -d "target" ]; then
  find target -depth -mindepth 1 -delete
  rmdir target 2>/dev/null || true
  cleaned_any=1
fi

# Remove local Codex audit scratch artifacts if present.
if [ -d ".codex_audit" ]; then
  find .codex_audit -depth -mindepth 1 -delete
  rmdir .codex_audit 2>/dev/null || true
  cleaned_any=1
fi

bytes_after="$(du -sk . 2>/dev/null | awk '{print $1}')"
freed_kb=$((bytes_before - bytes_after))
if [ "$freed_kb" -lt 0 ]; then
  freed_kb=0
fi

if [ "$cleaned_any" -eq 1 ]; then
  echo "Cleanup complete. Freed approximately ${freed_kb} KB."
else
  echo "Nothing to clean."
fi
