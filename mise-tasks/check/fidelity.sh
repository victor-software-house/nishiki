#!/usr/bin/env bash
#MISE description="Every Shiki grammar gets Shiki's scopes on every non-whitespace byte of its sample"
#MISE depends=["oracle:fidelity"]
# Loads each grammar oracle:fidelity wrote through the public API, and fails
# on the first input whose scopes differ from Shiki's.
# target/fidelity/report.md holds a row per input.
set -euo pipefail
NISHIKI_FIDELITY="$PWD/target/fidelity" cargo test --release --locked \
  --no-default-features --features textmate,json --lib \
  textmate::tests::fidelity -- --ignored --nocapture
