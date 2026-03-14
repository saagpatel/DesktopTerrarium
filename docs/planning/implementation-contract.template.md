# Implementation Contract

## Goal

Reach production-ready completion for Desktop Terrarium with deterministic build/test behavior, truthful quality gates, and release trust controls.

## In Scope

- Deterministic local/CI execution with stable `CARGO_TARGET_DIR` behavior.
- Rust-native quality and performance gates.
- Security and supply-chain checks (`cargo deny`, `cargo audit`, CodeQL, dependency review).
- Optional-path release signing and notarization workflow.
- Release provenance, checksums, and SBOM outputs.
- Runbooks for go/no-go, rollback, and staged rollout.

## Out of Scope

- New gameplay feature expansions beyond current terrarium experience.
- Platform distribution channels that require legal or account setup not present in this repo.
- Apple credential provisioning itself.

## Acceptance Criteria

- CI required checks execute deterministically on PR and merge queue events.
- Native perf baseline is non-placeholder and enforced by workflow.
- Release workflow produces dual-arch + universal artifacts plus checksums and SBOM.
- Signing/notarization path is implemented and documented as optional gated execution.
- Docs allow a new operator to run quality, release, and rollback procedures without implicit knowledge.

## Release Definition

A release candidate is considered shippable when required CI and native perf checks pass, supply-chain checks pass, and release artifact generation succeeds with verification outputs.

## Stop / Escalate Policy

Stop and escalate only for:

- Missing required credentials/secrets for signing or notarization.
- Conflicting PM direction on release channel policy.
- Security gates failing with unresolved High/Critical findings.
- Reproducibility/provenance failures on release artifacts.
