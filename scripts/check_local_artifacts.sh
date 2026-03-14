#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

tracked_files=()
while IFS= read -r path; do
  tracked_files+=("$path")
done < <(git ls-files)
offenders=()

for path in "${tracked_files[@]}"; do
  case "$path" in
    *.DS_Store|target/*|.codex_audit/*)
      offenders+=("$path")
      ;;
  esac
done

if [ "${#offenders[@]}" -gt 0 ]; then
  echo "Local artifact guard failed."
  echo "Remove these tracked files before committing:"
  printf '  - %s\n' "${offenders[@]}"
  exit 1
fi

echo "Local artifact guard passed."
