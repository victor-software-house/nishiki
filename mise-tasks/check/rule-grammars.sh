#!/usr/bin/env bash
#MISE description="Each rule-engine language's tables match its oracle/<language>-grammar.ts"
#MISE tools={bun="1.4.4"}
# Runs every rule-engine generator, oracle/<language>-grammar.ts.
# oracle/rule-grammar.ts is the library they share, not a generator.
# oracle/php-family-grammar.ts is the one PHP's and Hack's generators share.
# Its tests in oracle/rule-grammar.test.ts run first.
# Each generator regenerates its language's tables under src/ in place.
# It formats them as `mise run format` does.
# The check fails when a file under src/ then differs from the committed one.
# The generators and their output cannot drift apart.
# Bun fetches Shiki from npm.
# The check is therefore not part of the offline verify.
set -euo pipefail

bun test oracle/rule-grammar.test.ts

for generator in oracle/*-grammar.ts; do
  case "$generator" in
    oracle/rule-grammar.ts | oracle/php-family-grammar.ts) continue ;;
  esac
  bun "$generator" >/dev/null
done
changed=$(git status --porcelain -- src)
if [ -n "$changed" ]; then
  echo "$changed" >&2
  echo "src/ differs from what the oracle/<language>-grammar.ts generators write;" \
    "run each with bun" >&2
  exit 1
fi
