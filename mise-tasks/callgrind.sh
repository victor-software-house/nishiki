#!/usr/bin/env bash
#MISE description="Instruction counts of lexing the oracle fixtures, against another commit"
# `mise run callgrind [base] [languages...]`, base `origin/main` by default,
# languages the rule-engine ones and Markdown by default. Both commits run
# examples/lex.rs over this tree's fixtures under callgrind, whose counts do
# not depend on the machine's load. Both build with one codegen unit, so a
# change that only moves code between units, which shifts counts by up to a
# few percent, does not read as a change: the counts are a review signal,
# while the wall-clock bench keeps the default profile users get. Prints a
# Markdown table of millions of instructions and head/base, and fails when a
# language rises more than CALLGRIND_TOLERANCE_PCT (0.5). Needs valgrind from the bench profile:
# MISE_ENV=ci,bench in CI, MISE_ENV=dev,bench locally (Linux only).
set -euo pipefail
base="${1:-origin/main}"
shift || true
languages=("$@")
if [ "${#languages[@]}" -eq 0 ]; then
  languages=(python java css kotlin make go ruby csharp markdown)
fi
tolerance="${CALLGRIND_TOLERANCE_PCT:-0.5}"
dir=target/callgrind
rm -rf "$dir/base"
mkdir -p "$dir/base/examples"
git archive "$base" | tar -x -C "$dir/base"
cp examples/lex.rs "$dir/base/examples/lex.rs"
fixtures="$PWD/tests/oracle"
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1
CARGO_TARGET_DIR="$dir/target-base" cargo build --quiet --release --locked \
  --manifest-path "$dir/base/Cargo.toml" --example lex
CARGO_TARGET_DIR="$dir/target-head" cargo build --quiet --release --locked --example lex
count() {
  valgrind --tool=callgrind --callgrind-out-file=/dev/null "$1" "$fixtures" "$2" 2>&1 |
    sed -n 's/.*Collected : *\([0-9]*\).*/\1/p'
}
echo "| language | base (M) | head (M) | head/base |"
echo "|:--|--:|--:|--:|"
failed=()
for language in "${languages[@]}"; do
  b=$(count "$dir/target-base/release/examples/lex" "$language")
  h=$(count "$dir/target-head/release/examples/lex" "$language")
  awk -v l="$language" -v b="$b" -v h="$h" 'BEGIN { printf "| %s | %.1f | %.1f | %.4f |\n", l, b / 1e6, h / 1e6, h / b }'
  if awk -v b="$b" -v h="$h" -v t="$tolerance" 'BEGIN { exit !(h > b * (1 + t / 100)) }'; then
    failed+=("$language")
  fi
done
if [ "${#failed[@]}" -gt 0 ]; then
  echo "instructions rose more than ${tolerance}% in: ${failed[*]}" >&2
  exit 1
fi
