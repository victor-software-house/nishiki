#!/usr/bin/env bash
#MISE description="Per-line latency on the corpus: steady p50, p99 and maximum, and first contact"
#MISE env={CARGO_TARGET_DIR = "target/bench"}
# Not part of verify: timings depend on the machine (bench/AGENTS.md).
# Arguments go to the program: `mise run bench:latency --tier medium python`
# measures Python's medium files; `--passes N` sets the passes (11).
set -euo pipefail
cargo run --quiet --locked --release --manifest-path bench/Cargo.toml --bin latency -- "$@"
