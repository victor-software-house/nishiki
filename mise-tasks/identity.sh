#!/usr/bin/env bash
#MISE description="Compare the tokens of edited oracle fixtures with another commit"
# For a change that must not change output, such as a refactor or a speed
# change: `mise run identity [base]`, base `origin/main` by default. Both
# commits lex this tree's fixtures and the same edited copies of them
# (examples/identity.rs); the task prints the documents whose tokens differ
# and fails when there are any. A language the base lacks shows up as added
# lines only, and passes.
#
# A change that means to change output names its languages in
# IDENTITY_CHANGES, such as `IDENTITY_CHANGES="python java"`: documents of
# those languages may differ, and any other difference still fails. CI reads
# them from a pull request's `identity: changes <languages>` line.
set -euo pipefail
. "$(dirname "$0")/lib/commit-target.sh"
base="${1:-origin/main}"
dir=target/identity
base_target=$(commit_target "$dir" base "$base")
head_target=$(commit_target "$dir" head HEAD)
keep_commit_targets "$dir" "$base_target" "$head_target"
rm -rf "$dir/base"
mkdir -p "$dir/base"
git archive "$base" | tar -x -C "$dir/base"
mkdir -p "$dir/base/examples"
cp examples/identity.rs "$dir/base/examples/identity.rs"
# The example builds in its own package, which a base from before it lacks.
# Such a base builds the copy above as its own example.
base_manifest="$dir/base/Cargo.toml"
if [ -f "$dir/base/examples/Cargo.toml" ]; then
  base_manifest="$dir/base/examples/Cargo.toml"
fi
fixtures="$PWD/tests/oracle"
CARGO_TARGET_DIR="$base_target" cargo run --quiet --release --locked \
  --manifest-path "$base_manifest" --example identity -- "$fixtures" >"$dir/base.txt"
CARGO_TARGET_DIR="$head_target" cargo run --quiet --release --locked \
  --package nishiki-examples --example identity -- "$fixtures" >"$dir/head.txt"
if diff "$dir/base.txt" "$dir/head.txt" >"$dir/diff.txt"; then
  echo "identical to $base: $(tail -n 1 "$dir/head.txt")"
  exit 0
fi
cat "$dir/diff.txt"
declared=" ${IDENTITY_CHANGES:-} "
undeclared=()
while read -r language; do
  case "$declared" in
  *" $language "*) ;;
  *) undeclared+=("$language") ;;
  esac
done < <(sed -n 's|^< \([^/ ]*\)/.*|\1|p' "$dir/diff.txt" | sort -u)
if [ "${#undeclared[@]}" -gt 0 ]; then
  echo "output differs from $base in: ${undeclared[*]}; declare it with IDENTITY_CHANGES, or in CI with a pull request line \`identity: changes <languages>\`" >&2
  exit 1
fi
echo "differs from $base only in the declared languages:${declared% }"
