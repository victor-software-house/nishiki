# aa-crit.sh <runs> <work-dir>: bench-methodology task 3.2 on one host.
# Builds the throughput bench once, then runs its medium tier <runs> times
# unpinned and <runs> times pinned to the last CPU, alternating, each run
# saved as its own Criterion baseline. Leaves results.tgz and DONE.
set -u
runs=$1; work=$2
cd "$work/src"
export PATH=$HOME/.cargo/bin:$PATH CARGO_TARGET_DIR=$work/tgt CRITERION_HOME=$work/crit
mkdir -p "$work/log"
T0=$(date +%s)
command -v cargo >/dev/null || curl -sSf https://sh.rustup.rs | sh -s -- -y -q --profile minimal --default-toolchain none >/dev/null 2>&1
cargo bench --locked --manifest-path bench/Cargo.toml --bench throughput --no-run > "$work/log/build.txt" 2>&1 || { echo BUILD_FAIL; touch "$work/DONE"; exit 1; }
bin=$(find "$CARGO_TARGET_DIR/release/deps" -maxdepth 1 -type f -perm -u+x -name 'throughput-*' | head -1)
echo "build $(( $(date +%s) - T0 ))s $bin" > "$work/log/progress.txt"
pin=$(( $(nproc) - 1 ))
for r in $(seq 1 "$runs"); do
  "$bin" --bench 'lex/.*/medium' --save-baseline "free-$r" > "$work/log/free-$r.txt" 2>&1
  taskset -c "$pin" "$bin" --bench 'lex/.*/medium' --save-baseline "pin-$r" > "$work/log/pin-$r.txt" 2>&1
  echo "run $r $(( $(date +%s) - T0 ))s" >> "$work/log/progress.txt"
done
{ date -u +%FT%TZ; grep -m1 "model name" /proc/cpuinfo; nproc; lscpu | grep -E "Thread|Core|Socket|MHz"; free -m | head -2; uname -r; uptime; } > "$work/log/host.txt" 2>&1
cd "$work"
find crit tgt/criterion -name estimates.json -path '*-[0-9]*/*' 2>/dev/null > list.txt
tar czf results.tgz log $(cat list.txt)
touch DONE
