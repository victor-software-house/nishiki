#!/usr/bin/env bash
#MISE description="The character classes are what oracle/classes.ts measures"
#MISE tools={bun="1.4.4"}
# oracle/classes.ts writes its fact file and its Rust table in place.
# The check fails when either then differs from the committed file.
# Bun fetches Shiki from npm.
# The check is therefore not part of the offline verify.
set -euo pipefail

files=(oracle/facts/classes.json src/classes/table.rs)
bun oracle/classes.ts >/dev/null
changed=$(git status --porcelain -- "${files[@]}")
if [ -n "$changed" ]; then
  echo "$changed" >&2
  echo "these files differ from what oracle/classes.ts writes; run bun oracle/classes.ts" >&2
  exit 1
fi
