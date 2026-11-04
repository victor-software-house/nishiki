#!/usr/bin/env bash
#MISE description="The bench workspace is formatted, passes its lints, and runs each benchmark once; the comparison workspace is formatted and linted"
#MISE env={CARGO_TARGET_DIR = "target/bench"}
# verify does not build bench/, its own workspace, so the Callgrind
# workflow's Bench job runs this on a pull request labelled `bench`.
# Criterion's --test runs each benchmark once, without timing it; nothing
# here is timed against a bound, so wall time cannot fail the job.
set -euo pipefail
cargo ${RUSTFMT_TOOLCHAIN:-} fmt --manifest-path bench/Cargo.toml -- --check
cargo clippy --locked --manifest-path bench/Cargo.toml --all-targets -- -D warnings
# The comparison workspace, its own workspace too, is formatted and linted
# only: running it needs Bun and Shiki's references, and giallo stays off
# because its licence keeps it to the bench box (bench/compare/README.md).
cargo ${RUSTFMT_TOOLCHAIN:-} fmt --manifest-path bench/compare/Cargo.toml -- --check
CARGO_TARGET_DIR=target/compare cargo clippy --locked --manifest-path bench/compare/Cargo.toml --all-targets -- -D warnings
cargo bench --locked --manifest-path bench/Cargo.toml -- --test
# The latency program runs once on the small tier with three passes; its
# table shows in the log, and no figure in it can fail the job.
cargo run --quiet --locked --release --manifest-path bench/Cargo.toml --bin latency -- --passes 3 --tier small
# The cold-start program runs five processes for Python's small file, two
# counted after the three dropped rounds; its table shows in the log, and no
# figure in it can fail the job.
cargo run --quiet --locked --release --manifest-path bench/Cargo.toml --bin cold -- --processes 5 python
