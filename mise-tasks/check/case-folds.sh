#!/usr/bin/env bash
#MISE description="The fold table is what oracle/case-folds.ts measures"
#MISE tools={bun="1.4.4"}
# src/textmate/regex/folds.rs is measured on Oniguruma as Shiki bundles it;
# this fails when the table and the script that writes it disagree. Bun fetches
# Shiki from npm, so the check is not part of the offline verify.
set -euo pipefail

out=$(mktemp)
trap 'rm -f "$out"' EXIT
bun oracle/case-folds.ts "$out" >/dev/null
if ! cmp -s "$out" src/textmate/regex/folds.rs; then
  echo "src/textmate/regex/folds.rs differs from what oracle/case-folds.ts writes; run bun oracle/case-folds.ts" >&2
  exit 1
fi
