#!/usr/bin/env bash
set -euo pipefail

release_ref="${1:?release ref required}"
notarize="${2:-false}"

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

dist_dir="dist/${release_ref}"
pkg_name="DesktopTerrarium-macos-universal"
work_dir=".release-work/${release_ref}/signed"
keychain_path="${work_dir}/build.keychain"

mkdir -p "$work_dir"
tar -xzf "$dist_dir/${pkg_name}.tar.gz" -C "$work_dir"

: "${MACOS_CERT_P12_B64:?MACOS_CERT_P12_B64 is required for signing}"
: "${MACOS_CERT_PASSWORD:?MACOS_CERT_PASSWORD is required for signing}"
: "${MACOS_SIGNING_IDENTITY:?MACOS_SIGNING_IDENTITY is required for signing}"
: "${MACOS_KEYCHAIN_PASSWORD:?MACOS_KEYCHAIN_PASSWORD is required for signing}"

cleanup() {
  security delete-keychain "$keychain_path" >/dev/null 2>&1 || true
  rm -f "$work_dir/cert.p12"
}
trap cleanup EXIT

echo "$MACOS_CERT_P12_B64" | base64 --decode > "$work_dir/cert.p12"
security create-keychain -p "$MACOS_KEYCHAIN_PASSWORD" "$keychain_path"
security set-keychain-settings -lut 21600 "$keychain_path"
security unlock-keychain -p "$MACOS_KEYCHAIN_PASSWORD" "$keychain_path"
security import "$work_dir/cert.p12" -k "$keychain_path" -P "$MACOS_CERT_PASSWORD" -T /usr/bin/codesign -T /usr/bin/security
security set-key-partition-list -S apple-tool:,apple: -s -k "$MACOS_KEYCHAIN_PASSWORD" "$keychain_path"
security list-keychains -d user -s "$keychain_path"
security default-keychain -d user -s "$keychain_path"
security find-identity -v -p codesigning "$keychain_path" >/dev/null

codesign --force --timestamp --options runtime \
  --sign "$MACOS_SIGNING_IDENTITY" \
  "$work_dir/${pkg_name}/desktop_terrarium"

if [[ "$notarize" == "true" ]]; then
  : "${APPLE_ID:?APPLE_ID is required for notarization}"
  : "${APPLE_TEAM_ID:?APPLE_TEAM_ID is required for notarization}"
  : "${APPLE_APP_SPECIFIC_PASSWORD:?APPLE_APP_SPECIFIC_PASSWORD is required for notarization}"

  ditto -c -k --keepParent "$work_dir/${pkg_name}" "$work_dir/${pkg_name}-notary.zip"
  xcrun notarytool submit "$work_dir/${pkg_name}-notary.zip" \
    --apple-id "$APPLE_ID" \
    --team-id "$APPLE_TEAM_ID" \
    --password "$APPLE_APP_SPECIFIC_PASSWORD" \
    --wait

  xcrun stapler staple "$work_dir/${pkg_name}/desktop_terrarium"
  spctl --assess --type execute --verbose "$work_dir/${pkg_name}/desktop_terrarium"
fi

tar -C "$work_dir" -czf "$dist_dir/${pkg_name}.tar.gz" "$pkg_name"
(
  cd "$dist_dir"
  shasum -a 256 "${pkg_name}.tar.gz" "${pkg_name}.spdx.json" > SHA256SUMS
)
