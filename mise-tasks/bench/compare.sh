#!/usr/bin/env bash
#MISE description="Compare nishiki with syntect, tree-sitter-highlight and Shiki on the benchmark corpus"
#MISE tools={bun="1.4.4"}
#MISE env={CARGO_TARGET_DIR = "target/compare"}
# Not part of verify: timings depend on the machine, and the comparators
# build C (bench/compare/README.md). Writes Shiki's reference colours, then
# runs the harness; arguments go to it, such as `--tier large rust`. giallo
# joins only on the bench box: `COMPARE_FEATURES=giallo mise run
# bench:compare`, since its licence keeps it out of anything distributed.
set -euo pipefail
bun bench/compare/shiki.ts reference
cargo run --quiet --locked --release --manifest-path bench/compare/Cargo.toml \
  --features "${COMPARE_FEATURES:-}" -- "$@"
