#!/usr/bin/env bash
#MISE description="Each benchmark corpus file matches its row in bench/corpus.toml"
#MISE tools = { taplo = "0.10.0" }
# A figure from the corpus means something only if every run reads the same
# bytes (bench/AGENTS.md; openspec bench-methodology, design decision 2).
# Those bytes must be under a licence that allows them here.
# The check fails on any of these:
# - a file's size or SHA-256 differs from its row;
# - a row has no file, or a file has no row;
# - a size falls outside its tier;
# - a source is not pinned to a commit;
# - a licence is not on the list below;
# - the manifest's digest differs from the digest of the files.
#
# The digest stands for the corpus version.
# Two pull requests that each add files both raise version from N to N+1.
# Git takes that identical line without a conflict, so the version alone
# merged silently four times on 2026-11-05.
# Each pull request writes a different digest on the line below it, so the
# same merge now conflicts there.
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

# The SHA-256 of one line per file, its path and its SHA-256, sorted by path.
digest_of_files() {
  local file
  while IFS= read -r file; do
    printf '%s\t%s\n' "${file#"$root/"}" "$(sum "$file")"
  done < <(find "$root" -type f ! -name README.md ! -name .ignore ! -name .gitattributes | LC_ALL=C sort) >"$tmp"
  sum "$tmp"
}
tmp=$(mktemp)
trap 'rm -f "$tmp"' EXIT

problems=()
# Every row's path, each between line feeds, for a membership test without a
# pipe.
# With `printf | grep -q`, grep can exit early.
# printf then dies of SIGPIPE, and pipefail reads that as no match
# (a merge-group run, 2026-11-04).
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
    grammar) low=1024 high=1048576 ;;
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

recorded=$(taplo get -f "$manifest" digest)
actual=$(digest_of_files)
if [ "$recorded" != "$actual" ]; then
  problems+=("$manifest: digest is $recorded, but the files give $actual")
  problems+=("  raise version past main's, then set digest = \"$actual\"")
fi

if [ "${#problems[@]}" -gt 0 ]; then
  printf '%s\n' "${problems[@]}" >&2
  exit 1
fi
