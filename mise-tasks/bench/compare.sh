#!/usr/bin/env bash
#MISE description="Compare nishiki with syntect, tree-sitter-highlight, Shiki and Pierre on the benchmark corpus"
#MISE tools={bun="1.4.4"}
#MISE env={CARGO_TARGET_DIR = "target/compare"}
#MISE depends=["check:bun-lockfile"]
# Not part of verify, because timings depend on the machine.
# The comparators also build C (bench/compare/README.md).
# It writes Shiki's reference colours and github-dark as a Zed theme for
# @pierre/highlights, then runs the harness.
# `bun bench/compare/shiki.ts time` times Shiki and Pierre themselves.
# Arguments go to the harness, such as `--tier large rust`.
# giallo joins only on the bench box, with
# `COMPARE_FEATURES=giallo mise run bench:compare`.
# Its licence keeps it out of anything distributed.
set -euo pipefail
bun bench/compare/shiki.ts reference
cargo run --quiet --locked --release --manifest-path bench/compare/Cargo.toml \
  --features "${COMPARE_FEATURES:-}" -- --pierre-theme target/compare/pierre-theme.json
cargo run --quiet --locked --release --manifest-path bench/compare/Cargo.toml \
  --features "${COMPARE_FEATURES:-}" -- "$@"
