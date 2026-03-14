# Execution Profiles

## Plan Profile (stable-first)

Use stable Codex capabilities for mandatory flow. Treat multi-agent tooling as optional acceleration.

## Build Profile

- Use `./scripts/env/with_deterministic_env.sh` for all cargo commands.
- Keep weather mode `full` by default.
- For low-risk fallback runs, set `TERRARIUM_WEATHER_MODE=safe_disable`.

## Release Profile

- Use `release-macos` workflow.
- Always generate checksums + SBOM + attestation.
- Enable signing/notarization only when required secrets are configured.
