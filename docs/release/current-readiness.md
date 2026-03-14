# Current Readiness

Last validated: 2026-03-14

## Scope

This document tracks the currently validated readiness state for the desktop terrarium app on macOS.

- Platform scope: macOS first
- Current target: local release candidate
- Not yet covered here: production signing, notarization, or broader runtime parity validation on Linux and Windows

## Local Release Candidate Status

Status: ready for local release-candidate use on macOS

Validated on the current branch state with:

- `./scripts/smoke/run_local_smoke.sh`
- `./.codex/scripts/run_verify_commands.sh`
- `./scripts/release/rehearse_unsigned_release.sh v0.1.0-readiness`
- `./scripts/release/smoke_packaged_app.sh v0.1.0-readiness`

Validated outcomes:

- App launches with deterministic smoke controls and overlay-driven state visibility.
- Repo-defined verification commands pass.
- Unsigned universal package, checksums, and SBOM outputs are generated successfully.
- Packaged binary launches and completes the scripted smoke flow.

## Remaining Work Before Production Publish

- Run signing flow with Developer ID credentials.
- Run notarization path if release policy requires it.
- Confirm final publish workflow against the exact release tag intended for distribution.

## Notes

- `cargo deny` is configured to ignore duplicate-version warnings so the gate stays focused on actionable failures in this Bevy/wgpu-based dependency graph.
- Legacy web perf artifacts remain in the repo for historical workflow compatibility, but they are not part of the desktop release gate.
