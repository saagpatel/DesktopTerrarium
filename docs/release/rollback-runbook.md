# Rollback Runbook

## Trigger Conditions

- Crash or startup failure exceeds release thresholds.
- Data integrity risk is detected.
- Security issue is discovered in the shipped artifact.

## Rollback Steps

1. Identify last known good release tag.
2. Communicate rollback start to stakeholders.
3. Re-point distribution to the known good tag artifacts.
4. Verify checksum and provenance for rollback artifacts.
5. Run smoke test on rollback artifact.
6. Announce rollback complete.

### Command Checklist

1. Verify artifact integrity:
   - `cd dist/<known-good-tag> && shasum -a 256 -c SHA256SUMS`
2. Validate rollback artifact launches within timeout:
   - `tar -xzf DesktopTerrarium-macos-universal.tar.gz`
   - `timeout 15 ./DesktopTerrarium-macos-universal/desktop_terrarium`
3. Confirm release metadata points to known good tag before re-enabling distribution.

## Post-Rollback Checks

- Verify startup, save/load, and basic interaction flow.
- Confirm no new critical incidents in the next 30 minutes.
- Capture incident timeline and corrective actions.
- Record measured recovery time objective (RTO) and state-compatibility outcome.
