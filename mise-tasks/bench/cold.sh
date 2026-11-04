#!/usr/bin/env bash
#MISE description="Cold start on the benchmark corpus: fresh processes, median and p90"
#MISE env={CARGO_TARGET_DIR = "target/bench"}
# Not part of verify: timings depend on the machine (bench/AGENTS.md).
# Arguments go to the program: `mise run bench:cold --tier medium python`
# measures Python's medium files; `--processes N` sets the samples (31).
set -euo pipefail
cargo run --quiet --locked --release --manifest-path bench/Cargo.toml --bin cold -- "$@"
