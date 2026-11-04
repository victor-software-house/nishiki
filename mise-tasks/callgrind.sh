#!/usr/bin/env bash
#MISE description="Instruction counts of lexing the oracle fixtures, against another commit"
# `mise run callgrind [base] [languages...]`, base `origin/main` by default,
# languages every one with oracle fixtures by default, built with the
# `textmate` tier. Both commits run examples/lex.rs under callgrind, whose
# counts do not depend on the machine's load, over the same files: this
# tree's fixtures that the base has too. A language the base has no fixtures
# for is listed as new. The runs go in parallel, one valgrind per core.
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
# fails when a language rises more than CALLGRIND_TOLERANCE_PCT (0.5). Then
# prints, for information, what one pass over each language's fixtures
# allocates on each side (examples/memory.rs). Needs
# valgrind from the bench profile: MISE_ENV=ci,bench in CI,
# MISE_ENV=dev,bench locally (Linux only).
set -euo pipefail
base="${1:-origin/main}"
shift || true
languages=("$@")
# Every directory of fixtures is a language, named as `Language::from_name`
# reads it; `themes/` holds no fixtures.
if [ "${#languages[@]}" -eq 0 ]; then
  for language_dir in tests/oracle/*/; do
    language=$(basename "$language_dir")
    if compgen -G "$language_dir*.txt" >/dev/null; then
      languages+=("$language")
    fi
  done
fi
tolerance="${CALLGRIND_TOLERANCE_PCT:-0.5}"
dir=target/callgrind
rm -rf "$dir/base"
mkdir -p "$dir/base/examples"
git archive "$base" | tar -x -C "$dir/base"
cp examples/lex.rs "$dir/base/examples/lex.rs"
# The memory counts use this tree's harness and counter on both sides, so a
# base whose nishiki-alloc-count is older than this tree's still builds them.
cp examples/memory.rs "$dir/base/examples/memory.rs"
rm -rf "$dir/base/nishiki-alloc-count/src"
cp -R nishiki-alloc-count/src "$dir/base/nishiki-alloc-count/src"
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
# default sixteen, which the table shows for information only. The memory
# counts do not depend on the layout, so only the default build has them.
for units in 1 16; do
  examples=(--example lex)
  if [ "$units" = 16 ]; then
    examples+=(--example memory)
  fi
  CARGO_PROFILE_RELEASE_CODEGEN_UNITS=$units CARGO_TARGET_DIR="$dir/target-base-$units" \
    cargo build --quiet --release --locked --manifest-path "$dir/base/Cargo.toml" \
    "${base_features[@]}" "${examples[@]}"
  CARGO_PROFILE_RELEASE_CODEGEN_UNITS=$units CARGO_TARGET_DIR="$dir/target-head-$units" \
    cargo build --quiet --release --locked "${head_features[@]}" "${examples[@]}"
done
# Counts one build (base-1, head-1, base-16 or head-16) on one language into
# counts/<build>.<language>. valgrind runs one thread, so the runs go in
# parallel, one per core; callgrind's counts do not depend on the load.
counts="$dir/counts"
rm -rf "$counts"
mkdir -p "$counts"
count() {
  valgrind --tool=callgrind --callgrind-out-file=/dev/null --collect-atstart=no \
    --toggle-collect='*lex::measured*' "$dir/target-$1/release/examples/lex" "$fixtures" "$2" 2>&1 |
    sed -n 's/.*Collected : *\([0-9]*\).*/\1/p' >"$counts/$1.$2"
}
export -f count
export dir fixtures counts
for language in "${languages[@]}"; do
  if [ -n "$(ls -A "$fixtures/$language")" ]; then
    printf '%s %s\n' base-1 "$language" head-1 "$language" base-16 "$language" head-16 "$language"
  fi
done | xargs -P "$(nproc)" -n 2 bash -c 'count "$1" "$2"' _
echo "| language | base (M) | head (M) | head/base | base, 16 units (M) | head, 16 units (M) | head/base, 16 units (not gated) |"
echo "|:--|--:|--:|--:|--:|--:|--:|"
failed=()
notes=()
for language in "${languages[@]}"; do
  if [ -z "$(ls -A "$fixtures/$language")" ]; then
    echo "| $language | new | | | | | |"
    continue
  fi
  b=$(cat "$counts/base-1.$language")
  h=$(cat "$counts/head-1.$language")
  b16=$(cat "$counts/base-16.$language")
  h16=$(cat "$counts/head-16.$language")
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
# What one pass over each language's fixtures allocates, from fresh
# highlighters (examples/memory.rs): exact counts, for information only.
echo
echo "Memory, for information: allocations, then KiB allocated, then the most KiB one document held at once."
echo
echo "| language | allocations, base | allocations, head | KiB, base | KiB, head | peak KiB, base | peak KiB, head |"
echo "|:--|--:|--:|--:|--:|--:|--:|"
for language in "${languages[@]}"; do
  if [ -z "$(ls -A "$fixtures/$language")" ]; then
    echo "| $language | new | | | | | |"
    continue
  fi
  read -r base_allocs base_bytes base_peak < <("$dir/target-base-16/release/examples/memory" "$fixtures" "$language")
  read -r head_allocs head_bytes head_peak < <("$dir/target-head-16/release/examples/memory" "$fixtures" "$language")
  if [ -z "${base_peak:-}" ] || [ -z "${head_peak:-}" ]; then
    echo "examples/memory.rs printed no counts for $language" >&2
    exit 1
  fi
  awk -v l="$language" -v b_n="$base_allocs" -v h_n="$head_allocs" -v b_bytes="$base_bytes" -v h_bytes="$head_bytes" -v b_peak="$base_peak" -v h_peak="$head_peak" 'BEGIN {
    printf "| %s | %d | %d | %.1f | %.1f | %.1f | %.1f |\n", l, b_n, h_n, b_bytes / 1024, h_bytes / 1024, b_peak / 1024, h_peak / 1024
  }'
done
if [ "${#failed[@]}" -gt 0 ]; then
  echo "instructions rose more than ${tolerance}% in: ${failed[*]}" >&2
  exit 1
fi
