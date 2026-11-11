#!/usr/bin/env bash
#MISE description="The Unicode sets' facts are what oracle/unicode-tables.ts measures"
#MISE tools={bun="1.4.4"}
# oracle/unicode-tables.ts writes its fact file in place.
# The check fails when it then differs from the committed file.
# codegen/ writes src/textmate/regex/unicode.rs from the facts, and verify's
# `mise run codegen -- --check` holds the table to them.
# Bun fetches Shiki from npm.
# The check is therefore not part of the offline verify.
set -euo pipefail

files=(oracle/facts/unicode-sets.json)
bun oracle/unicode-tables.ts >/dev/null
changed=$(git status --porcelain -- "${files[@]}")
if [ -n "$changed" ]; then
  echo "$changed" >&2
  echo "these files differ from what oracle/unicode-tables.ts writes;" \
    "run bun oracle/unicode-tables.ts" >&2
  exit 1
fi
