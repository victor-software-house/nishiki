#!/usr/bin/env bash
#MISE description="The case folds are what oracle/case-folds.ts measures"
#MISE tools={bun="1.4.4"}
# oracle/case-folds.ts writes its fact file and its Rust table in place.
# The check fails when either then differs from the committed file.
# Bun fetches Shiki from npm.
# The check is therefore not part of the offline verify.
set -euo pipefail

files=(oracle/facts/case-folds.json src/textmate/regex/folds.rs)
bun oracle/case-folds.ts >/dev/null
changed=$(git status --porcelain -- "${files[@]}")
if [ -n "$changed" ]; then
  echo "$changed" >&2
  echo "these files differ from what oracle/case-folds.ts writes; run bun oracle/case-folds.ts" >&2
  exit 1
fi
