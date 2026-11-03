#!/usr/bin/env bash
#MISE description="Throughput, streaming latency and cold start on the oracle fixtures"
#MISE env={CARGO_TARGET_DIR = "target/bench"}
# Not part of verify: timings depend on the machine. Compare runs on one
# machine, and measure on a quiet one. The `textmate` tier is on, so every
# oracle fixture is measured.
set -euo pipefail
cargo run --release --locked --features textmate --example bench -- "$@"
