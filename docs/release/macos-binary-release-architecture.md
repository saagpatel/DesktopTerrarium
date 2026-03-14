# macOS Binary Release Architecture

## Artifact Flow

1. Dispatch release workflow with explicit tag (`vX.Y.Z`) and release options.
2. Run preflight release gate (`run_release_gate.sh`) on the selected tag.
3. Build `aarch64-apple-darwin` binary.
4. Build `x86_64-apple-darwin` binary.
5. Assemble universal binary using `lipo`.
6. Package archive with assets.
7. Generate SBOM and checksums.
8. Attest artifact provenance.
9. Optionally sign and notarize.
10. Publish release artifacts (signed releases only).

## Scripts

- `scripts/release/macos_build_arch.sh`
- `scripts/release/macos_assemble_universal.sh`
- `scripts/release/macos_supply_chain.sh`
- `scripts/release/macos_sign_notarize.sh`

## Optional Secret Inputs

- `MACOS_CERT_P12_B64`
- `MACOS_CERT_PASSWORD`
- `MACOS_SIGNING_IDENTITY`
- `MACOS_KEYCHAIN_PASSWORD`
- `APPLE_ID`
- `APPLE_TEAM_ID`
- `APPLE_APP_SPECIFIC_PASSWORD`
