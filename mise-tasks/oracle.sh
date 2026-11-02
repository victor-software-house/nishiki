#!/usr/bin/env bash
#MISE description="Regenerate the Shiki fixtures under tests/oracle/"
#MISE tools={bun="1.4.4"}
# Fetches every input in oracle/inputs.ts at its pinned commit and writes
# Shiki's scopes for it. Run it after changing the inputs or the Shiki pin in
# oracle/generate.ts, and commit the fixtures it writes.
set -euo pipefail
bun run oracle/generate.ts
