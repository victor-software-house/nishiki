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
base="${1:-origin/main}"
dir=target/identity
rm -rf "$dir/base"
mkdir -p "$dir/base"
git archive "$base" | tar -x -C "$dir/base"
mkdir -p "$dir/base/examples"
cp examples/identity.rs "$dir/base/examples/identity.rs"
fixtures="$PWD/tests/oracle"
CARGO_TARGET_DIR="$dir/target-base" cargo run --quiet --release --locked \
  --manifest-path "$dir/base/Cargo.toml" --example identity -- "$fixtures" >"$dir/base.txt"
CARGO_TARGET_DIR="$dir/target-head" cargo run --quiet --release --locked \
  --example identity -- "$fixtures" >"$dir/head.txt"
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
