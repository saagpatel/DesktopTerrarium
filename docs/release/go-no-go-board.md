# Go / No-Go Board

## Local Release Candidate Gates

- CI quality checks
- Supply-chain checks (`cargo deny`, `cargo audit`)
- Native perf check
- CodeQL check
- Release package generation (dual-arch + universal)
- Checksum + SBOM + attestation outputs
- Local smoke script passes
- Packaged-app smoke passes

## Production Publish Gates

- Signed binary output
- Notarization, if release policy requires notarized binaries

## Decision Rule

- Local release candidate is `Go` only if all local release candidate gates are green.
- Production publish is `Go` only if all local release candidate gates are green and all required production publish gates are green.
- `No-Go` if any required gate fails or is not run.

See `docs/release/current-readiness.md` for the current validated state of this branch.
