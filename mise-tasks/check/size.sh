#!/usr/bin/env bash
#MISE description="Fail when the size example's sections at a tier grow past its budgets"
# Builds the `size` example with the release profile at each tier in
# [workspace.metadata.size] of the root Cargo.toml.
# It reads the sections with binutils' size.
# `text` is `.text`.
# `data` is `.rodata`, `.data.rel.ro` and `.data`.
# A tier fails when either one exceeds its budget by more than
# tolerance-percent.
# The budgets are linux-x64 numbers, so other platforms skip the check.
set -euo pipefail
platform="$(uname -s)-$(uname -m)"
if [[ "${platform}" != "Linux-x86_64" ]]; then
  echo "check:size: the budgets are linux-x64 numbers; skipped on ${platform}" >&2
  exit 0
fi
config="$(cargo metadata --format-version 1 --no-deps --locked | jq -c '.metadata.size')"
tolerance="$(jq -r '.["tolerance-percent"]' <<<"${config}")"
status=0
while IFS=$'\t' read -r tier features text_budget data_budget; do
  binary="$(cargo build --locked --release --package nishiki-examples --example size \
    --no-default-features --features "${features}" --message-format json-render-diagnostics |
    jq -r 'select(.reason == "compiler-artifact" and .target.name == "size") | .executable')"
  read -r text data < <(size -A "${binary}" | awk '
    $1 == ".text" { text = $2 }
    $1 == ".rodata" || $1 == ".data.rel.ro" || $1 == ".data" { data += $2 }
    END { print text + 0, data + 0 }')
  if ((text == 0 || data == 0)); then
    echo "check:size: ${tier}: size -A found no .text or data sections in ${binary}" >&2
    status=1
    continue
  fi
  for section in text data; do
    if [[ ${section} == text ]]; then
      bytes=${text} budget=${text_budget}
    else
      bytes=${data} budget=${data_budget}
    fi
    limit=$((budget + budget * tolerance / 100))
    echo "check:size: ${tier} (${features}): ${section} ${bytes} bytes, budget ${budget}, limit ${limit}"
    if ((bytes > limit)); then
      echo "check:size: ${tier} ${section} is ${bytes} bytes, over its limit of ${limit}" >&2
      status=1
    fi
  done
done < <(jq -r '.tiers | to_entries[] | [.key, .value.features, .value.text, .value.data] | @tsv' <<<"${config}")
exit "${status}"
