# Autonomous Handoff Packet

## Mission Snapshot

- Project: Desktop Terrarium
- Objective: Complete production-readiness hardening and release operations.
- Primary branch policy: `codex/<type>/<slug>`.

## Ordered Execution Plan

1. Stabilize environment and CI determinism.
2. Run and validate quality/security gates.
3. Validate native perf baseline and threshold checks.
4. Build and verify release artifacts.
5. Execute release go/no-go checklist.

## Guardrails

- Do not bypass required checks.
- Do not delete or rewrite unrelated user changes.
- Escalate only on documented blockers in implementation contract.

## Evidence Package Required

- CI run links/results for quality, supply-chain, native-perf, and CodeQL.
- Release artifact bundle with checksums/SBOM/attestation.
- Rollback decision log and staged rollout status.
