#!/usr/bin/env bash
#MISE description="The bench workspace is formatted, passes its lints, and runs each benchmark once"
#MISE env={CARGO_TARGET_DIR = "target/bench"}
# verify does not build bench/, its own workspace, so the Callgrind
# workflow's Bench job runs this on a pull request labelled `bench`.
# Criterion's --test runs each benchmark once, without timing it.
set -euo pipefail
cargo ${RUSTFMT_TOOLCHAIN:-} fmt --manifest-path bench/Cargo.toml -- --check
cargo clippy --locked --manifest-path bench/Cargo.toml --all-targets -- -D warnings
cargo bench --locked --manifest-path bench/Cargo.toml -- --test
