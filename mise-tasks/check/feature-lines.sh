#!/usr/bin/env bash
#MISE description="check:features builds every language of the all list in Cargo.toml alone"
# check:features builds each language alone, one line per feature.
# A new language joins the all list in Cargo.toml.
# Nothing ties that list to the lines of check:features.
# A language can then miss its line unseen.
# The languages of the textmate tier share one engine.
# check:features builds them together, through the textmate tier.
# This check fails on each feature of the all list that has no line of its own.
set -euo pipefail

all=$(taplo get -f Cargo.toml features.all)
lines=$(sed -n 's/^cargo check .* --features \(lang-[a-z-]*\)$/\1/p' mise-tasks/check/features.sh)

status=0
while read -r feature; do
  if ! grep -qxF "$feature" <<<"$lines"; then
    echo "mise-tasks/check/features.sh: $feature has no line that builds it alone" >&2
    status=1
  fi
done <<<"$all"
exit "$status"
