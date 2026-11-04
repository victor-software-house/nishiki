#!/usr/bin/env bash
#MISE description="Wall-clock benchmarks on the benchmark corpus, with Criterion"
#MISE env={CARGO_TARGET_DIR = "target/bench"}
# Not part of verify, because timings depend on the machine
# (bench/AGENTS.md).
# The bench workspace reads bench/corpus/, with every language built in.
# Arguments go to Criterion.
# `mise run bench lex/python` runs only the benchmarks whose name holds the
# filter.
# `--save-baseline` and `--baseline` compare two runs on one machine.
set -euo pipefail
cargo bench --locked --manifest-path bench/Cargo.toml -- "$@"
