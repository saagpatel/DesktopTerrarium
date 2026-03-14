#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

summary_file=".artifacts/readiness-summary.md"
mkdir -p .artifacts

{
  echo "# Readiness Summary"
  echo
  echo "Generated: $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
  echo
  echo "## Contract"
  test -f docs/planning/implementation-contract.md && echo "- implementation-contract: present" || echo "- implementation-contract: missing"
  test -f docs/planning/definition-of-done.md && echo "- definition-of-done: present" || echo "- definition-of-done: missing"
  echo
  echo "## Baselines"
  test -f .perf-baselines/native.json && echo "- native perf baseline: present" || echo "- native perf baseline: missing"
  echo
  echo "## Release Docs"
  test -f docs/release/macos-binary-release-architecture.md && echo "- release architecture: present" || echo "- release architecture: missing"
  test -f docs/release/checklists/macos-staged-release-checklist.md && echo "- staged checklist: present" || echo "- staged checklist: missing"
} > "$summary_file"

echo "Wrote $summary_file"
