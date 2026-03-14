# macOS Staged Release Checklist

## Stage 0 - Preflight

- CI, security, and native perf gates are green.
- Release tag is created and release notes drafted.
- Local preflight command succeeds:
  - `./scripts/release/run_release_gate.sh`
- Local unsigned rehearsal succeeds:
  - `./scripts/release/rehearse_unsigned_release.sh <tag>-local-rehearsal`
- If signing is planned, required secret values are present in repo settings.

## Stage 1 - Build

- `aarch64-apple-darwin` artifact generated.
- `x86_64-apple-darwin` artifact generated.
- Workflow uses the exact requested tag (`vX.Y.Z`) as checkout ref.

## Stage 2 - Universal Package

- Universal binary assembled with `lipo`.
- Asset bundle included.
- `DesktopTerrarium-macos-universal.tar.gz` is generated in `dist/<tag>`.
- `./scripts/release/smoke_packaged_app.sh <tag>` passes against the packaged binary.

## Stage 3 - Supply Chain Outputs

- `SHA256SUMS` generated.
- SBOM output generated.
- Provenance attestation generated.
- `shasum -a 256 -c SHA256SUMS` passes for release files.

## Stage 4 - Trust Gate (Optional)

- Signing succeeds with Developer ID identity.
- Notarization succeeds when enabled.
- `spctl` assessment succeeds when notarization is enabled.
- Publish step is enabled only for signed runs (`sign=true`).

## Stage 5 - Publish and Monitor

- Release assets published.
- Rollback target is recorded.
- T+30m, T+4h, and T+24h verification checks complete.
