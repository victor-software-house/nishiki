#!/usr/bin/env bash
#MISE description="The Unicode sets are what oracle/unicode-tables.ts measures"
#MISE tools={bun="1.4.4"}
# oracle/unicode-tables.ts writes its fact file and its Rust table in place.
# The check fails when either then differs from the committed file.
# Bun fetches Shiki from npm.
# The check is therefore not part of the offline verify.
set -euo pipefail

files=(oracle/facts/unicode-sets.json src/textmate/regex/unicode.rs)
bun oracle/unicode-tables.ts >/dev/null
changed=$(git status --porcelain -- "${files[@]}")
if [ -n "$changed" ]; then
  echo "$changed" >&2
  echo "these files differ from what oracle/unicode-tables.ts writes;" \
    "run bun oracle/unicode-tables.ts" >&2
  exit 1
fi
