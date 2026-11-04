#!/usr/bin/env bash
#MISE description="Compare nishiki with syntect, tree-sitter-highlight and Shiki on the benchmark corpus"
#MISE tools={bun="1.4.4"}
#MISE env={CARGO_TARGET_DIR = "target/compare"}
# Not part of verify, because timings depend on the machine.
# The comparators also build C (bench/compare/README.md).
# It writes Shiki's reference colours, then runs the harness.
# Arguments go to the harness, such as `--tier large rust`.
# giallo joins only on the bench box, with
# `COMPARE_FEATURES=giallo mise run bench:compare`.
# Its licence keeps it out of anything distributed.
set -euo pipefail
bun bench/compare/shiki.ts reference
cargo run --quiet --locked --release --manifest-path bench/compare/Cargo.toml \
  --features "${COMPARE_FEATURES:-}" -- "$@"
