#!/usr/bin/env bash
#MISE description="Each rule-engine language's tables are what its oracle/<language>-grammar.ts writes"
#MISE tools={bun="1.4.4"}
# Runs every rule-engine generator, oracle/<language>-grammar.ts.
# oracle/rule-grammar.ts is the library they share, not a generator.
# Each generator regenerates its language's tables under src/ in place.
# It formats them as `mise run format` does.
# The check fails when a file under src/ then differs from the committed one.
# The generators and their output cannot drift apart.
# Bun fetches Shiki from npm.
# The check is therefore not part of the offline verify.
set -euo pipefail

for generator in oracle/*-grammar.ts; do
  if [ "$generator" = oracle/rule-grammar.ts ]; then
    continue
  fi
  bun "$generator" >/dev/null
done
changed=$(git status --porcelain -- src)
if [ -n "$changed" ]; then
  echo "$changed" >&2
  echo "src/ differs from what the oracle/<language>-grammar.ts generators write; run each with bun" >&2
  exit 1
fi
