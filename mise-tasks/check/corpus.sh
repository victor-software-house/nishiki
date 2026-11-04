#!/usr/bin/env bash
#MISE description="Each benchmark corpus file matches its row in bench/corpus.toml"
# A figure from the corpus means something only if every run reads the same
# bytes under a licence that allows them here (bench/AGENTS.md; openspec
# bench-methodology, design decision 2). Fails when a file's size or SHA-256
# differs from its row, a row has no file, a file has no row, a size falls
# outside its tier, a source is not pinned to a commit, or a licence is not
# on the list below.
set -euo pipefail

root=bench/corpus
manifest=bench/corpus.toml
# Permissive licences only, as SPDX expressions (AGENTS.md, invariant 2).
allowed='^(MIT|Apache-2\.0|BSD-2-Clause|BSD-3-Clause|ISC|Zlib|CC0-1\.0|Unlicense|PSF-2\.0|MIT OR Apache-2\.0|Unlicense OR MIT)$'

if command -v sha256sum >/dev/null; then
  sum() { sha256sum "$1" | cut -d' ' -f1; }
else
  sum() { shasum -a 256 "$1" | cut -d' ' -f1; }
fi

problems=()
# Every row's path, each between line feeds, for a membership test without a
# pipe: `printf | grep -q` lets grep exit early, printf then dies of SIGPIPE,
# and pipefail reads that as no match (a merge-group run, 2026-11-04).
listed=$'\n'
while IFS=$'\t' read -r path tier bytes sha source licence; do
  listed+="$path"$'\n'
  file="$root/$path"
  if [ ! -f "$file" ]; then
    problems+=("$path: listed but missing")
    continue
  fi
  size=$(wc -c <"$file" | tr -d ' ')
  [ "$size" = "$bytes" ] || problems+=("$path: $size bytes, the row says $bytes")
  [ "$(sum "$file")" = "$sha" ] || problems+=("$path: SHA-256 differs from its row")
  case "$tier" in
    small) low=1024 high=4096 ;;
    medium) low=16384 high=65536 ;;
    large) low=204800 high=1048576 ;;
    reply) low=4096 high=40960 ;;
    *) low=1 high=0 ;;
  esac
  if [ "$size" -lt "$low" ] || [ "$size" -gt "$high" ]; then
    problems+=("$path: $size bytes is outside the $tier tier")
  fi
  grep -qE '/[0-9a-f]{40}/' <<<"$source" || problems+=("$path: source is not pinned to a commit")
  grep -qE "$allowed" <<<"$licence" || problems+=("$path: licence $licence is not on the allowed list")
done < <(taplo get -f "$manifest" -o json | jq -r '.file[] | [.path, .tier, .bytes, .sha256, .source, .licence] | @tsv')

while IFS= read -r file; do
  path=${file#"$root/"}
  [ "$path" = README.md ] && continue
  [ "$path" = .ignore ] && continue
  [ "$path" = .gitattributes ] && continue
  [[ $listed == *$'\n'"$path"$'\n'* ]] || problems+=("$path: no row in $manifest")
done < <(find "$root" -type f | sort)

if [ "${#problems[@]}" -gt 0 ]; then
  printf '%s\n' "${problems[@]}" >&2
  exit 1
fi
