#!/usr/bin/env bash
#MISE description="Write tests/oracle/COVERAGE.md: how much of each hand-lexed grammar the oracle exercises"
#MISE tools={bun="1.4.4"}
# Tokenizes every oracle fixture and a fixed fuzz corpus through Shiki, with
# the hand-lexed grammars tagged, and counts the expressions and captures each
# one exercises. Run it after changing the oracle's inputs or fixtures, and
# commit the report it writes.
set -euo pipefail
bun run oracle/coverage.ts
