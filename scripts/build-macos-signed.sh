#!/usr/bin/env bash
# Build signé + notarisé de Breeze pour macOS (Developer ID, hors App Store).
set -euo pipefail
cd "$(dirname "$0")/.." || exit 1
if [ ! -f .env.signing ]; then
  echo "manque .env.signing — voir ~/Breeze-Signing/" >&2; exit 1
fi
# shellcheck disable=SC1091
source .env.signing
echo "Signature : $APPLE_SIGNING_IDENTITY"
echo "Notarisation : clé $APPLE_API_KEY"
cargo tauri build --bundles app,dmg
