# Sourced by the tasks that build another commit beside this tree.
#
# `commit_target <dir> <side> <commit>` prints the cargo target directory for
# building <commit> as <side> ("base" or "head") under <dir>: one per commit.
# The base tree comes from `git archive`, which dates every file at its
# commit, so cargo would take a directory that last built another commit as
# up to date and hand back that commit's binary. Each commit's own directory
# stays cached for the next run on the same commit.
commit_target() {
  local dir=$1 side=$2 commit=$3 sha
  if ! sha=$(git rev-parse --verify --quiet "$commit^{commit}"); then
    echo "$commit names no commit" >&2
    return 1
  fi
  printf '%s/%s/%s\n' "$dir" "$sha" "$side"
}

# `keep_commit_targets <dir> <target>…` removes every commit's directory under
# <dir> but those holding the given targets, so a kept machine holds the
# builds of the commits the task compares now and no others.
keep_commit_targets() {
  local dir=$1 entry target kept
  shift
  for entry in "$dir"/*; do
    [[ $(basename "$entry") =~ ^[0-9a-f]{40}$ ]] || continue
    kept=false
    for target in "$@"; do
      [[ $target == "$entry"/* ]] && kept=true
    done
    "$kept" || rm -rf "$entry"
  done
}
