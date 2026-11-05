#!/usr/bin/env bash
#MISE description="The README's Tier column and the crate docs' tier table match the tier lists in Cargo.toml"
# A language that leaves the textmate tier moves from Cargo.toml's textmate
# list to its all list.
# Its README row's Tier moves from `textmate` to `all`.
# Its name leaves the `textmate` row of the tier table in src/lib.rs.
# Nothing else ties the three.
# A row can then keep its old tier unseen.
# This check fails on each README row whose Tier differs from Cargo.toml.
# It also fails on each tier table row whose languages differ from the
# tier's list in Cargo.toml.
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

# The README's language names, each with its feature, to read the tier table
# by.
names=$(while IFS='|' read -r _ language _ feature _; do
  printf '%s\t%s\n' "$(echo "$language" | xargs)" "$(echo "$feature" | tr -d '` ')"
done <<<"$rows")

tiers=$(awk '/^\/\/! \| Feature \| Languages \|/ { table = 1; getline; next }
  table && /^\/\/! \|/ { print; next }
  table { exit }' src/lib.rs)
if [ -z "$tiers" ]; then
  echo "src/lib.rs has no tier table" >&2
  exit 1
fi

while IFS='|' read -r _ tier languages _; do
  tier=$(echo "$tier" | sed 's/(the default)//' | tr -d '` ')
  # `all` is every language but the textmate tier, in prose.
  if [ "$tier" = all ]; then
    continue
  fi
  want=$(taplo get -f Cargo.toml "features.$tier" | sort -u)
  # A name in backticks is a feature; any other name is a README language.
  have=$(echo "$languages" | sed 's/ and /, /g' | tr ',' '\n' | while read -r name; do
    name=$(echo "$name" | xargs)
    case "$name" in
      \`*\`) echo "$name" | tr -d '`' ;;
      *) awk -F'\t' -v name="$name" '$1 == name { print $2; found = 1 } END { if (!found) print "unknown:" name }' <<<"$names" ;;
    esac
  done | sort -u)
  if [ "$have" != "$want" ]; then
    echo "src/lib.rs: the $tier row differs from Cargo.toml's $tier list" >&2
    diff <(echo "$want") <(echo "$have") | sed -n 's/^< /  missing from the row: /p; s/^> /  not in Cargo.toml: /p' >&2
    status=1
  fi
done <<<"$tiers"
exit "$status"
