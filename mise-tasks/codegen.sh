#!/usr/bin/env bash
#MISE description="Write the Rust files codegen/ generates; --check fails on a stale one"
#MISE env={CARGO_TARGET_DIR = "target/codegen"}
# codegen/ is a workspace of its own (openspec rust-codegen, design section 3).
# It builds in target/codegen, apart from the root's builds, as bench/ builds in target/bench.
# nishiki::unstable compiles only with --cfg nishiki_unstable.
# A set RUSTFLAGS replaces codegen/.cargo/config.toml, so the cfg is appended to the caller's flags.
# CARGO_ENCODED_RUSTFLAGS replaces RUSTFLAGS too, so it must be unset.
set -euo pipefail
if [[ -n "${CARGO_ENCODED_RUSTFLAGS:-}" ]]; then
  echo "unset CARGO_ENCODED_RUSTFLAGS: it drops the --cfg nishiki_unstable codegen needs" >&2
  exit 1
fi
export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--cfg nishiki_unstable"
cargo run --quiet --locked --manifest-path codegen/Cargo.toml -- "$@"
