#!/usr/bin/env bash
#MISE description="Write target/fidelity/: every Shiki grammar, its sample's scopes and the engine's tables"
#MISE tools={bun="1.4.4"}
# Fetches Shiki's sample of every language Shiki bundles, and writes what
# check:fidelity compares. Nothing it writes is committed.
set -euo pipefail
rm -rf target/fidelity
bun run oracle/fidelity.ts target/fidelity
