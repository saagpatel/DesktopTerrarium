#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

mkdir -p .perf-results

if [[ -z "${CARGO_TARGET_DIR:-}" ]]; then
  export CARGO_TARGET_DIR="$(./scripts/env/with_deterministic_env.sh | sed -n 's/^CARGO_TARGET_DIR=//p')"
fi
mkdir -p "$CARGO_TARGET_DIR"

measure_stable_ms() {
  local sample_count="$1"
  shift

  python3 - "$sample_count" "$@" <<'PY'
import subprocess
import sys
import time

sample_count = int(sys.argv[1])
command = sys.argv[2:]
samples = []

for _ in range(sample_count):
    start = time.time()
    subprocess.run(command, check=True, stdout=subprocess.DEVNULL)
    end = time.time()
    samples.append(int((end - start) * 1000))

samples.sort()
print(samples[len(samples) // 2])
PY
}

# Warm caches once to reduce noisy first-run variance in CI and local measurements.
./scripts/env/with_deterministic_env.sh cargo build --workspace --all-targets --locked >/dev/null
./scripts/env/with_deterministic_env.sh cargo test --workspace --lib --locked >/dev/null

sample_count="${NATIVE_PERF_SAMPLES:-3}"

build_ms="$(measure_stable_ms "$sample_count" ./scripts/env/with_deterministic_env.sh cargo build --workspace --all-targets --locked)"
test_ms="$(measure_stable_ms "$sample_count" ./scripts/env/with_deterministic_env.sh cargo test --workspace --lib --locked)"

binary_path="${CARGO_TARGET_DIR:-target}/debug/desktop_terrarium"
if [[ ! -f "$binary_path" ]]; then
  binary_path="$(find "${CARGO_TARGET_DIR:-target}/debug" -maxdepth 2 -type f -name 'desktop_terrarium*' | head -n 1 || true)"
fi
if [[ -z "${binary_path:-}" || ! -f "$binary_path" ]]; then
  echo "Could not locate debug binary artifact for metric capture." >&2
  exit 1
fi
binary_bytes="$(wc -c < "$binary_path")"

python3 - <<PY
import json
from datetime import datetime, timezone

payload = {
    "buildMs": int(${build_ms}),
    "testMs": int(${test_ms}),
    "binaryBytes": int(${binary_bytes}),
    "sampleCount": int(${sample_count}),
    "capturedAt": datetime.now(timezone.utc).isoformat(),
    "binaryPath": "${binary_path}",
}

with open('.perf-results/native.json', 'w', encoding='utf-8') as f:
    payload["binaryArtifact"] = payload.pop("binaryPath").split("/")[-1]
    json.dump(payload, f, indent=2)
    f.write('\n')

print(json.dumps(payload, indent=2))
PY
