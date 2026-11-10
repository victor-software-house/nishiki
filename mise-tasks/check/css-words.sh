#!/usr/bin/env bash
#MISE description="The CSS word table is what oracle/css-words.ts writes"
#MISE tools={bun="1.4.4"}
# src/css/words.rs is generated from the css grammar's word lists.
# This check fails when the table and the script that writes it disagree.
# Bun fetches Shiki from npm.
# The check is therefore not part of the offline verify.
set -euo pipefail

out=$(mktemp)
trap 'rm -f "$out"' EXIT
bun oracle/css-words.ts "$out" >/dev/null
if ! cmp -s "$out" src/css/words.rs; then
  echo "src/css/words.rs differs from what oracle/css-words.ts writes;" \
    "run bun oracle/css-words.ts" >&2
  exit 1
fi
