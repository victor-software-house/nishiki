#!/usr/bin/env bash
#MISE description="Each browsable directory has its README, and its guide where it has rules"
# A reader browsing the repository on GitHub learns what a directory holds
# from its README, and the rules for working there from its AGENTS.md
# (AGENTS.md, Documentation; openspec/changes/archive/2026-11-03-repo-layout).
set -euo pipefail

both=(bench openspec oracle tests)
readme_only=(src mise-tasks)

missing=()
for dir in "${both[@]}" "${readme_only[@]}"; do
  [ -f "$dir/README.md" ] || missing+=("$dir/README.md")
done
for dir in "${both[@]}"; do
  [ -f "$dir/AGENTS.md" ] || missing+=("$dir/AGENTS.md")
done
if [ "${#missing[@]}" -gt 0 ]; then
  printf 'missing %s\n' "${missing[@]}" >&2
  exit 1
fi
