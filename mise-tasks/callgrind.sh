#!/usr/bin/env bash
#MISE description="Instruction counts of lexing the oracle fixtures, against another commit"
# `mise run callgrind [base] [languages...]`, base `origin/main` by default,
# languages the rule-engine and TextMate-engine ones and Markdown by default,
# built with the `textmate` tier. Both commits run examples/lex.rs under
# callgrind, whose counts do not depend on the machine's load, over the same
# files: this tree's fixtures that the base has too. A language the base has
# no fixtures for is listed as new.
#
# The gate builds both with one codegen unit, so a change that only moves
# code between units, which shifts counts by up to a few percent, does not
# read as a change: the counts are a review signal. The table also shows
# both at the default sixteen units, the layout users get, for information
# only: it never fails the task. Only the harness's second pass is counted,
# so the work a process does once, such as building each rule's pattern
# tables, stays out.
#
# Prints a Markdown table of millions of instructions and head/base, and
# fails when a language rises more than CALLGRIND_TOLERANCE_PCT (0.5). Needs
# valgrind from the bench profile: MISE_ENV=ci,bench in CI,
# MISE_ENV=dev,bench locally (Linux only).
set -euo pipefail
base="${1:-origin/main}"
shift || true
languages=("$@")
if [ "${#languages[@]}" -eq 0 ]; then
  languages=(c cmake cpp csharp css go java kotlin lua make markdown powershell python ruby scala sql swift zig)
fi
tolerance="${CALLGRIND_TOLERANCE_PCT:-0.5}"
dir=target/callgrind
rm -rf "$dir/base"
mkdir -p "$dir/base/examples"
git archive "$base" | tar -x -C "$dir/base"
cp examples/lex.rs "$dir/base/examples/lex.rs"
# Both binaries lex the same files: this tree's fixtures that the base has
# too, so inputs a pull request adds do not count as a change of code.
fixtures="$dir/fixtures"
rm -rf "$fixtures"
for language in "${languages[@]}"; do
  mkdir -p "$fixtures/$language"
  for file in tests/oracle/"$language"/*.txt; do
    if [ -f "$dir/base/$file" ]; then
      ln -s "$PWD/$file" "$fixtures/$language/"
    fi
  done
done
# The TextMate engine's languages need the `textmate` tier, which a base
# from before it lacks; its languages are then new there.
head_features=(--features textmate)
base_features=()
if grep -q '^textmate = ' "$dir/base/Cargo.toml"; then
  base_features=(--features textmate)
fi
# One build of each side at one codegen unit, which gates, and one at the
# default sixteen, which the table shows for information only.
for units in 1 16; do
  CARGO_PROFILE_RELEASE_CODEGEN_UNITS=$units CARGO_TARGET_DIR="$dir/target-base-$units" \
    cargo build --quiet --release --locked --manifest-path "$dir/base/Cargo.toml" \
    "${base_features[@]}" --example lex
  CARGO_PROFILE_RELEASE_CODEGEN_UNITS=$units CARGO_TARGET_DIR="$dir/target-head-$units" \
    cargo build --quiet --release --locked "${head_features[@]}" --example lex
done
count() {
  valgrind --tool=callgrind --callgrind-out-file=/dev/null --collect-atstart=no \
    --toggle-collect='*lex::measured*' "$1" "$fixtures" "$2" 2>&1 |
    sed -n 's/.*Collected : *\([0-9]*\).*/\1/p'
}
echo "| language | base (M) | head (M) | head/base | base, 16 units (M) | head, 16 units (M) | head/base, 16 units (not gated) |"
echo "|:--|--:|--:|--:|--:|--:|--:|"
failed=()
notes=()
for language in "${languages[@]}"; do
  if [ -z "$(ls -A "$fixtures/$language")" ]; then
    echo "| $language | new | | | | | |"
    continue
  fi
  b=$(count "$dir/target-base-1/release/examples/lex" "$language")
  h=$(count "$dir/target-head-1/release/examples/lex" "$language")
  b16=$(count "$dir/target-base-16/release/examples/lex" "$language")
  h16=$(count "$dir/target-head-16/release/examples/lex" "$language")
  if [ "${b:-0}" = 0 ] || [ "${h:-0}" = 0 ] || [ "${b16:-0}" = 0 ] || [ "${h16:-0}" = 0 ]; then
    echo "callgrind counted nothing for $language: lex::measured was not found" >&2
    exit 1
  fi
  awk -v l="$language" -v b="$b" -v h="$h" -v b16="$b16" -v h16="$h16" 'BEGIN {
    printf "| %s | %.1f | %.1f | %.4f | %.1f | %.1f | %.4f |\n", l, b / 1e6, h / 1e6, h / b, b16 / 1e6, h16 / 1e6, h16 / b16
  }'
  if awk -v b="$b" -v h="$h" -v t="$tolerance" 'BEGIN { exit !(h > b * (1 + t / 100)) }'; then
    failed+=("$language")
  fi
  if awk -v h="$h" -v h16="$h16" 'BEGIN { exit !(h16 > h * 1.03) }'; then
    notes+=("$(awk -v l="$language" -v h="$h" -v h16="$h16" 'BEGIN { printf "%s +%.1f%%", l, (h16 / h - 1) * 100 }')")
  fi
done
if [ "${#notes[@]}" -gt 0 ]; then
  echo
  echo "Note, not a failure: at sixteen codegen units the head runs more than 3% more instructions than at one in ${notes[*]}. A matcher its scan calls in another module may lack \`#[inline]\` (bench/AGENTS.md rule 13)."
fi
if [ "${#failed[@]}" -gt 0 ]; then
  echo "instructions rose more than ${tolerance}% in: ${failed[*]}" >&2
  exit 1
fi
