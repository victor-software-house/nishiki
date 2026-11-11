#!/usr/bin/env bash
#MISE description="The case folds' facts are what oracle/case-folds.ts measures"
#MISE tools={bun="1.4.4"}
# oracle/case-folds.ts writes its fact file in place.
# The check fails when it then differs from the committed file.
# codegen/ writes src/textmate/regex/folds.rs from the facts, and verify's
# `mise run codegen -- --check` holds the table to them.
# Bun fetches Shiki from npm.
# The check is therefore not part of the offline verify.
set -euo pipefail

files=(oracle/facts/case-folds.json)
bun oracle/case-folds.ts >/dev/null
changed=$(git status --porcelain -- "${files[@]}")
if [ -n "$changed" ]; then
  echo "$changed" >&2
  echo "these files differ from what oracle/case-folds.ts writes; run bun oracle/case-folds.ts" >&2
  exit 1
fi
