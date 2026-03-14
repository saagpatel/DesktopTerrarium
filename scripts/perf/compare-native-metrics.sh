#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 2 ]]; then
  echo "usage: $0 <baseline.json> <current.json>"
  exit 2
fi

baseline="$1"
current="$2"

BUILD_MAX_RATIO="${NATIVE_BUILD_MAX_RATIO:-0.40}"
TEST_MAX_RATIO="${NATIVE_TEST_MAX_RATIO:-0.30}"
BINARY_MAX_RATIO="${NATIVE_BINARY_MAX_RATIO:-0.15}"

python3 - "$baseline" "$current" "$BUILD_MAX_RATIO" "$TEST_MAX_RATIO" "$BINARY_MAX_RATIO" <<'PY'
import json
import sys

baseline_path, current_path, build_ratio, test_ratio, binary_ratio = sys.argv[1:]

with open(baseline_path, "r", encoding="utf-8") as f:
    base = json.load(f)
with open(current_path, "r", encoding="utf-8") as f:
    cur = json.load(f)

limits = {
    "buildMs": float(build_ratio),
    "testMs": float(test_ratio),
    "binaryBytes": float(binary_ratio),
}

errors = []
for metric, limit in limits.items():
    b = float(base[metric])
    c = float(cur[metric])
    ratio = (c - b) / b if b else 0.0
    print(f"{metric}: baseline={b:.2f}, current={c:.2f}, ratio={ratio:.4f}, limit={limit:.4f}")
    if ratio > limit:
        errors.append((metric, ratio, limit))

if errors:
    print("Native perf regression detected:")
    for metric, ratio, limit in errors:
        print(f"- {metric}: {ratio:.4f} > {limit:.4f}")
    sys.exit(1)
PY
