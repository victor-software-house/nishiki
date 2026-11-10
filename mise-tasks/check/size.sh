#!/usr/bin/env bash
#MISE description="Fail when a size program's sections at a tier grow past its budgets"
#MISE wait_for=["lint"]
# Builds one program with the release profile at each tier in
# [workspace.metadata.size] of the root Cargo.toml.
# A tier names its package and binary, or builds the `size` example of
# nishiki-examples. Its features, when it names any, follow
# --no-default-features.
# It reads the sections with binutils' size.
# `text` is `.text`.
# `data` is `.rodata`, `.data.rel.ro` and `.data`.
# A tier fails when either one exceeds its budget by more than
# tolerance-percent.
# The budgets are linux-x64 numbers, so other platforms skip the check.
# It waits for lint when verify runs both, so their compiles of the whole crate
# do not overlap.
# mise.toml says why, beside test:doc, which waits for this task.
set -euo pipefail
platform="$(uname -s)-$(uname -m)"
if [[ "${platform}" != "Linux-x86_64" ]]; then
  echo "check:size: the budgets are linux-x64 numbers; skipped on ${platform}" >&2
  exit 0
fi
config="$(cargo metadata --format-version 1 --no-deps --locked | jq -c '.metadata.size')"
tolerance="$(jq -r '.["tolerance-percent"]' <<<"${config}")"
status=0
while IFS=$'\t' read -r tier package kind name text_budget data_budget features; do
  build=(cargo build --locked --release --package "${package}" "--${kind}" "${name}")
  build+=(--no-default-features)
  if [[ -n "${features}" ]]; then
    build+=(--features "${features}")
  fi
  binary="$("${build[@]}" --message-format json-render-diagnostics |
    jq -r --arg name "${name}" \
      'select(.reason == "compiler-artifact" and .target.name == $name) | .executable')"
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
    echo "check:size: ${tier} (${package} ${name}${features:+ ${features}}):" \
      "${section} ${bytes} bytes, budget ${budget}, limit ${limit}"
    if ((bytes > limit)); then
      echo "check:size: ${tier} ${section} is ${bytes} bytes, over its limit of ${limit}" >&2
      status=1
    fi
  done
done < <(
  jq -r '.tiers | to_entries[] | [
      .key,
      .value.package // "nishiki-examples",
      if .value.bin then "bin" else "example" end,
      .value.bin // "size",
      .value.text,
      .value.data,
      .value.features // ""
    ] | @tsv' <<<"${config}"
)
exit "${status}"
