#!/usr/bin/env bash
#MISE description="This repository's own checks"
# The published crate is the only copy of nishiki's source its users can
# read, so it must carry the licence texts and the third-party notices.
set -euo pipefail

listed=$(cargo package --list --locked --allow-dirty)
for file in README.md LICENSE-APACHE LICENSE-MIT THIRD-PARTY-NOTICES.md; do
  if ! grep -qxF "$file" <<<"$listed"; then
    echo "the packaged crate lacks $file; add it to include in Cargo.toml" >&2
    exit 1
  fi
done

# CODING_STANDARDS 3.4: multiline text is an indoc! or formatdoc! document,
# never a literal with two or more escaped line feeds. A literal of line feeds
# alone, such as a "\n\n" separator, holds no text and passes. git grep reads
# only tracked files; comment lines and generated files are skipped.
any='([^"\\]|\\.)*'
text='([^"\\]|\\[^n])'
lf='\\n'
literal="\"$any($lf$any$lf$any$text|$lf$any$text$any$lf|$text$any$lf$any$lf)"
generated=()
while IFS= read -r file; do
  generated+=(":!$file")
done < <(git grep -lF 'do not edit.' -- '*.rs')
if found=$(git grep -nE "$literal" -- 'src/*.rs' 'tests/*.rs' "${generated[@]}" |
  grep -vE '^[^:]+:[0-9]+:[[:space:]]*//'); then
  echo "$found" >&2
  echo "these literals spell multiline text with escaped line feeds;" \
    "write it with indoc! or formatdoc! (CODING_STANDARDS 3.4)" >&2
  exit 1
fi
