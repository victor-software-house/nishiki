#!/usr/bin/env bash
#MISE description="The README's Tier column matches the all and textmate lists in Cargo.toml"
# A language that leaves the textmate tier moves from Cargo.toml's textmate
# list to its all list.
# Its README row's Tier moves from `textmate` to `all`.
# Nothing else ties the two.
# A row can then keep its old tier unseen.
# This check fails on each row whose Tier differs from Cargo.toml.
set -euo pipefail

all=$(taplo get -f Cargo.toml features.all)
textmate=$(taplo get -f Cargo.toml features.textmate)

rows=$(awk '/^\| Language \| Variant \| Feature \| Tier \|/ { table = 1; getline; next }
  table && /^\|/ { print; next }
  table { exit }' README.md)
if [ -z "$rows" ]; then
  echo "README.md has no language table with a Tier column" >&2
  exit 1
fi

status=0
while IFS='|' read -r _ language _ feature tier _; do
  language=$(echo "$language" | xargs)
  feature=$(echo "$feature" | tr -d '` ')
  tier=$(echo "$tier" | tr -d '` ')
  if grep -qxF "$feature" <<<"$textmate"; then
    want=textmate
  elif grep -qxF "$feature" <<<"$all"; then
    want=all
  else
    echo "README.md: $language's feature $feature is in neither all nor textmate in Cargo.toml" >&2
    status=1
    continue
  fi
  if [ "$tier" != "$want" ]; then
    echo "README.md: $language's Tier is $tier, but Cargo.toml puts $feature in $want" >&2
    status=1
  fi
done <<<"$rows"
exit "$status"
