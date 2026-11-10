#!/usr/bin/env bash
#MISE description="The engine's test grammars are what oracle/engine-grammars.ts writes"
#MISE tools={bun="1.4.4"}
# oracle/engine-grammars.ts writes tests/engine/grammars/ in place.
# It writes each grammar as indented JSON, the tables it links and the sources.
# The check fails when a file there then differs from the committed one.
# Bun fetches Shiki from npm.
# The check is therefore not part of the offline verify.
set -euo pipefail

bun oracle/engine-grammars.ts >/dev/null
changed=$(git status --porcelain -- tests/engine/grammars)
if [ -n "$changed" ]; then
  echo "$changed" >&2
  echo "tests/engine/grammars/ differs from what oracle/engine-grammars.ts writes;" \
    "run bun oracle/engine-grammars.ts" >&2
  exit 1
fi
