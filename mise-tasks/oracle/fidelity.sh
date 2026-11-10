#!/usr/bin/env bash
#MISE description="Write target/fidelity/: every Shiki grammar, its sample's scopes and the engine's tables"
#MISE tools={bun="1.4.4"}
#MISE depends=["check:bun-lockfile", "check:sources"]
# Reads Shiki's sample of every language Shiki bundles from the
# nishiki-sources checkout in NISHIKI_SOURCES, and writes what check:fidelity
# compares. Nothing it writes is committed.
set -euo pipefail
rm -rf target/fidelity
bun run oracle/fidelity.ts target/fidelity
