#!/usr/bin/env bash
#MISE description="Compare the tokens of edited oracle fixtures with another commit"
# For a change that must not change output, such as a refactor or a speed
# change: `mise run identity [base]`, base `origin/main` by default. Both
# commits lex this tree's fixtures and the same edited copies of them
# (examples/identity.rs); the task prints the documents whose tokens differ
# and fails when there are any. A language the base lacks shows up as added
# lines.
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
diff "$dir/base.txt" "$dir/head.txt"
echo "identical to $base: $(tail -n 1 "$dir/head.txt")"
