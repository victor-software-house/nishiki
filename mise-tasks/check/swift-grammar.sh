#!/usr/bin/env bash
#MISE description="Swift's rule-engine tables are what oracle/swift-grammar.ts writes"
#MISE tools={bun="1.4.4"}
# Regenerates src/swift's tables in place.
# It formats them as `mise run format` does.
# It fails when they differ from the committed files.
# The generator and its output cannot drift apart.
# Bun fetches Shiki from npm.
# The check is therefore not part of the offline verify.
set -euo pipefail

bun oracle/swift-grammar.ts >/dev/null
changed=$(git status --porcelain -- src/swift)
if [ -n "$changed" ]; then
  echo "$changed" >&2
  echo "src/swift differs from what oracle/swift-grammar.ts writes; run bun oracle/swift-grammar.ts" >&2
  exit 1
fi
