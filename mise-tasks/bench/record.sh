#!/usr/bin/env bash
#MISE description="A recorded benchmark session, written to bench/results/namespace-8x16/<date>-<sha8>.json"
#MISE tools={bun="1.4.4"}
#MISE env={CARGO_TARGET_DIR = "target/bench"}
# Runs only on the reference host class, Namespace 8x16, on a clean tree
# (bench/AGENTS.md): `mise run bench:record`.
# bench/record.ts says what a session runs.
set -euo pipefail
bun bench/record.ts "$@"
