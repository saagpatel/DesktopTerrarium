#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

state_dir="${TERRARIUM_STATE_DIR:-$(mktemp -d "${TMPDIR:-/tmp}/desktop-terrarium-smoke.XXXXXX")}"
export TERRARIUM_STATE_DIR="$state_dir"
export TERRARIUM_SMOKE_SCRIPT="${TERRARIUM_SMOKE_SCRIPT:-1}"

echo "Using smoke state dir: $TERRARIUM_STATE_DIR"
./scripts/env/with_deterministic_env.sh cargo run "$@"

state_file="$TERRARIUM_STATE_DIR/state.json"
if [[ ! -f "$state_file" ]]; then
  echo "Smoke run did not produce a persisted state file: $state_file" >&2
  exit 1
fi

python3 - <<'PY' "$state_file"
import json
import pathlib
import sys

state_path = pathlib.Path(sys.argv[1])
payload = json.loads(state_path.read_text(encoding="utf-8"))
plants = ", ".join(
    f"slot {idx}:{plant['species']}@stage{plant['stage']}"
    for idx, plant in enumerate(payload["plants"])
)
print("Smoke state summary:")
print(f"  path: {state_path}")
print(f"  weather: {payload['weather']}")
print(f"  time phase: {payload['time_of_day_phase']}")
print(f"  plants: {plants}")
print(f"  total active secs: {payload['total_active_secs']}")
PY
