#!/usr/bin/env bash
#MISE description="COVERAGE.md matches oracle/coverage.ts, and complete grammars stay complete"
#MISE tools={bun="1.4.4"}
# Measures the oracle's coverage again into a temporary file and fails when
# it differs from the committed report, or when oracle/coverage.ts fails: a
# wrong waiver, or a grammar listed complete with a gap. Bun fetches Shiki
# from npm, so the check is not part of the offline verify.
set -euo pipefail

out=$(mktemp)
trap 'rm -f "$out"' EXIT
bun run oracle/coverage.ts "$out" >/dev/null
if ! cmp -s "$out" tests/oracle/COVERAGE.md; then
  diff -u tests/oracle/COVERAGE.md "$out" >&2 || true
  echo "tests/oracle/COVERAGE.md differs from what oracle/coverage.ts writes;" \
    "run mise run oracle:coverage" >&2
  exit 1
fi
