#!/usr/bin/env bash
#MISE description="The unsafe test-support crate's tests under Miri"
#MISE env={CARGO_TARGET_DIR = "target/miri"}
# nishiki forbids unsafe code; nishiki-alloc-count holds the one unsafe
# implementation the tests need, so its tests run under Miri to catch
# undefined behaviour (CODING_STANDARDS 3.3). Miri runs on the pinned nightly.
# Miri builds with its own rustc driver, so it ignores a RUSTC_WRAPPER such as
# CI's build cache, and it warns when one is set.
set -euo pipefail
unset RUSTC_WRAPPER
cargo "+${RUST_NIGHTLY}" miri test --locked -p nishiki-alloc-count
