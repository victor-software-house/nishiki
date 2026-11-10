#!/usr/bin/env bash
#MISE description="Every Shiki grammar gets Shiki's scopes on every non-whitespace byte of its sample"
#MISE depends=["oracle:fidelity"]
# Runs the engine on what oracle:fidelity wrote, and fails on the first
# language whose sample differs from Shiki's.
# target/fidelity/report.md holds a row per language.
set -euo pipefail
NISHIKI_FIDELITY="$PWD/target/fidelity" cargo test --release --locked \
  --no-default-features --features textmate,json --lib \
  textmate::tests::fidelity -- --ignored --nocapture
