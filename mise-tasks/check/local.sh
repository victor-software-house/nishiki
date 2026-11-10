#!/usr/bin/env bash
#MISE description="This repository's own checks"
#MISE depends=[
#MISE   "check:bench-report",
#MISE   "check:bun-imports",
#MISE   "check:corpus",
#MISE   "check:feature-lines",
#MISE   "check:layout",
#MISE   "check:literals",
#MISE   "check:tiers",
#MISE   "check:traces",
#MISE ]
# The published crate is the only copy of nishiki's source its users can
# read, so it must carry the licence texts and the third-party notices.
set -euo pipefail

listed=$(cargo package --list --locked --allow-dirty)
for file in README.md LICENSE-APACHE LICENSE-MIT THIRD-PARTY-NOTICES.md; do
  if ! grep -qxF "$file" <<<"$listed"; then
    echo "the packaged crate lacks $file; add it to include in Cargo.toml" >&2
    exit 1
  fi
done
