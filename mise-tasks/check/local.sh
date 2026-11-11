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

# The crate holds src/ and the root's own files, and no grammar.
# Grammars and the other test data stay in the repository, under licences
# the crate's own do not cover.
# Cargo adds Cargo.toml.orig, Cargo.lock and .cargo_vcs_info.json itself.
root=(
  .cargo_vcs_info.json
  Cargo.lock
  Cargo.toml
  Cargo.toml.orig
  LICENSE-APACHE
  LICENSE-MIT
  README.md
  THIRD-PARTY-NOTICES.md
)
outside=$(grep -v '^src/' <<<"$listed" | grep -vxF -f <(printf '%s\n' "${root[@]}") || true)
grammars=$(grep -E '^src/.*\.(json|plist|tmLanguage)$' <<<"$listed" || true)
packaged=$(printf '%s\n%s\n' "$outside" "$grammars" | grep -v '^$' || true)
if [[ -n "$packaged" ]]; then
  echo "the packaged crate holds a file outside src/ and the root's own, or a grammar:" >&2
  echo "$packaged" >&2
  echo "drop it from include in Cargo.toml" >&2
  exit 1
fi
