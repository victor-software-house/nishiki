# Sourced by callgrind.sh and identity.sh.
#
# Their builds run without mbx (MBX_DISABLE=1).
# mbx can compile one source two ways at one codegen unit.
# The two binaries differ by about 2% in instruction count.
# A comparison would then measure mbx, not the code.
export MBX_DISABLE=1

# `commit_target <dir> <side> <commit>` prints a cargo target directory.
# The directory is `<dir>/<commit sha>/<side>`, where <side> is "base" or "head".
# Each commit gets its own directory.
# `git archive` dates every file at its commit.
# In a shared directory, cargo would treat another commit's build as up to date.
# A repeat run on the same commit reuses its directory.
commit_target() {
  local dir=$1 side=$2 commit=$3 sha
  if ! sha=$(git rev-parse --verify --quiet "$commit^{commit}"); then
    echo "$commit names no commit" >&2
    return 1
  fi
  printf '%s/%s/%s\n' "$dir" "$sha" "$side"
}

# `keep_commit_targets <dir> <target>…` removes old build directories.
# It removes every commit directory under <dir> that holds none of the targets.
# Disk use then stays at the two builds the task compares.
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
