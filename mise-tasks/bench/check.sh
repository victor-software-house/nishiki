#!/usr/bin/env bash
#MISE description="The bench workspace is formatted, passes its lints, and runs each benchmark once; the comparison workspace is formatted and linted"
#MISE env={CARGO_TARGET_DIR = "target/bench"}
# verify does not build bench/, which is its own workspace.
# The Callgrind workflow's Bench job runs this on a pull request labelled
# `bench`.
# Criterion's --test runs each benchmark once, without timing it.
# Nothing here is timed against a bound.
# Wall time cannot fail the job.
set -euo pipefail
cargo ${RUSTFMT_TOOLCHAIN:-} fmt --manifest-path bench/Cargo.toml -- --check
cargo clippy --locked --manifest-path bench/Cargo.toml --all-targets -- -D warnings
# The comparison workspace is its own workspace too.
# It is formatted and linted only.
# Running it needs Bun and Shiki's references.
# giallo stays off, because its licence keeps it to the bench box
# (bench/compare/README.md).
cargo ${RUSTFMT_TOOLCHAIN:-} fmt --manifest-path bench/compare/Cargo.toml -- --check
CARGO_TARGET_DIR=target/compare cargo clippy --locked --manifest-path bench/compare/Cargo.toml --all-targets -- -D warnings
cargo bench --locked --manifest-path bench/Cargo.toml -- --test
# The latency program runs once on the small tier with three passes.
# Its table shows in the log.
# No figure in it can fail the job.
cargo run --quiet --locked --release --manifest-path bench/Cargo.toml --bin latency -- --passes 3 --tier small
# The cold-start program runs five processes for Python's small file.
# It drops the first three rounds and counts two.
# Its table shows in the log.
# No figure in it can fail the job.
cargo run --quiet --locked --release --manifest-path bench/Cargo.toml --bin cold -- --processes 5 python
# The tab-separated modes bench/record.ts reads run once each on one file.
# A change to either program's output shows here.
cargo run --quiet --locked --release --manifest-path bench/Cargo.toml --bin latency -- --passes 2 --tier small --tsv python
cargo run --quiet --locked --release --manifest-path bench/Cargo.toml --bin cold -- --processes 4 --tsv python
# The stream program replays one trace of each kind twice.
# It fails when a document's final lines differ from lexing its text.
# No figure in it can fail the job.
cargo run --quiet --locked --release --manifest-path bench/Cargo.toml --bin stream -- --passes 2 uv-docker regex-determinize src-limits
