# Automation Playbook

Use worktree-isolated automation runs for low-noise readiness reporting.

## Suggested recurring tasks

- Daily readiness summary: `scripts/ops/readiness_summary.sh`
- Weekly native perf baseline review: `scripts/perf/native-smoke.sh` + `scripts/perf/compare-native-metrics.sh`
- Weekly release gate dry run: `scripts/release/run_release_gate.sh`
- Weekly unsigned release rehearsal: `scripts/release/rehearse_unsigned_release.sh`

## Output target

- `.artifacts/readiness-summary.md`
