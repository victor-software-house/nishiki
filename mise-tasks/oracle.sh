#!/usr/bin/env bash
#MISE description="Regenerate the Shiki fixtures under tests/oracle/"
#MISE tools={bun="1.4.4"}
#MISE depends=["check:oracle-lockfile", "check:sources"]
# Reads every input in oracle/inputs.ts from the nishiki-sources checkout in
# NISHIKI_SOURCES and writes Shiki's scopes for it. Run it after changing the
# inputs or the Shiki pin in oracle/generate.ts, and commit the fixtures it
# writes.
set -euo pipefail
bun run oracle/generate.ts
