#!/usr/bin/env bash
set -euo pipefail

COMMANDS_FILE="${1:-.codex/verify.commands}"

if [[ ! -f "$COMMANDS_FILE" ]]; then
  echo "missing verify commands file: $COMMANDS_FILE" >&2
  exit 1
fi

if [[ -f .codex/actions/_artifact_env.sh ]]; then
  # shellcheck source=/dev/null
  source .codex/actions/_artifact_env.sh
fi

failed=0
while IFS= read -r cmd || [[ -n "$cmd" ]]; do
  [[ -z "${cmd//[[:space:]]/}" ]] && continue
  [[ "$cmd" =~ ^[[:space:]]*# ]] && continue
  echo ">> $cmd"
  if ! bash -lc "$cmd"; then
    failed=1
    break
  fi
done < "$COMMANDS_FILE"

exit "$failed"
