#!/usr/bin/env bash
#MISE description="Stream-trace latency: setup, first delta, steady p50, p99, maximum, whole file"
#MISE env={CARGO_TARGET_DIR = "target/bench"}
# Not part of verify: timings depend on the machine (bench/AGENTS.md).
# Arguments go to the program: `mise run bench:stream reply` replays the
# reply traces; `--passes N` sets the passes (11).
set -euo pipefail
cargo run --quiet --locked --release --manifest-path bench/Cargo.toml --bin stream -- "$@"
