# bench

Rules for measuring nishiki, beside the root guide's invariants.
[README.md](README.md) says what each figure answers and how to run it.
`mise run bench` runs the bench workspace's Criterion benchmarks on `corpus/`.
`mise run bench:latency` gives per-line latency.
`mise run bench:cold` gives cold start.
`targets.toml` holds what results are judged against.
`results/` holds recorded runs.
The reasons for these rules are in the [`bench-methodology` design](../openspec/changes/archive/2026-11-04-bench-methodology/design.md).

1. Wall time never gates CI (design decision 10).
   Shared CI runners move it by more than any change we care about.
   - A local run compares two commits of one change on one machine.
     It is never recorded.
   - A recorded run comes from the reference host class, Namespace 8x16, unpinned (design decision 11).
     `mise run bench:record` checks the host and refuses any other.
   - No host class keeps two runs of one build within 2%.
     Recorded wall-clock figures are therefore indicative, and speed decisions rest on Callgrind instruction counts.
   - A box for a measurement is created for the run and deleted by the run's own script, pass or fail.
2. Before a paid run, estimate its minutes and cost.
   A run expected to cost more than about $15 waits for the operator's approval.
3. A recorded run is data first (design decision 12).
   - `mise run bench:record` writes `results/<host class>/<date>-<sha8>.json`.
   - The raw harness output stays beside it, unchanged, as `<date>-<sha8>.txt`.
   - `mise run bench:report` renders the record's page, `<date>-<sha8>.md`, and the summary in `README.md`.
   - Nobody writes a record, its page or the summary by hand.
     `check:bench-report` fails when either differs from its JSON.
   - The record names the commit, the corpus version, the host, the toolchain, the anchor drift and its bound, and the compute.
     Every figure carries its rounds, median, minimum, maximum and spread.
4. Keep the commit a record cites reachable.
   Push the tag `bench/<full sha>` when the record is committed.
   A squash or a restack can leave the commit on no branch.
5. A pull request that claims a speed change shows two things, in this order (design decision 17).
   - First, the Callgrind job's table at one codegen unit, which decides.
   - Second, when wall time matters, Criterion on the reference host.
     Save the base with `--save-baseline`, and compare the head with `--baseline`.
     Run them in the order base, head, head, base.
     The claim holds when both pairs move the same way and Criterion's interval for the change excludes zero.
   - Build each side in its own clean target directory.
     A tree from `git archive` keeps the commit's file times, so cargo can reuse an older build.
     On 2026-11-02 a stale before-build turned a 1.05 gain into a reported 1.3.
6. A result regresses when the statistic its target names is worse than the last recorded result on the same host class.
   It must be worse by more than `tolerance_pct`, and by at least the unit's floor.
   A regression blocks the change unless its pull request states why it is accepted.
7. A target in `targets.toml` changes only by a dated decision.
   The comment beside the target cites it.
   A target not set yet is listed with the reason.
8. Every comparison with another engine runs in `bench/compare/` (design decisions 3 and 13).
   It is a workspace of its own and is never published.
   No comparator becomes a dependency of the crate or of `bench/`.
9. A speed change never changes output.
   The oracle, colour, fuzz and limit tests stay green, and the run's report says so.
10. Termination on hostile input is a test, not a benchmark (`tests/it/limits/<language>.rs`).
    Each language's generated hostile lines must finish within the default limits.
11. `mise run bench` builds with Cargo's default release profile on purpose.
    A downstream binary builds nishiki that way, so the figures are what it sees.
    Do not add LTO or `codegen-units` to the root profile to improve them.
12. `mise run callgrind` gates on instructions with both sides built at one codegen unit.
    - The Callgrind workflow runs it on a pull request labelled `bench`.
    - The task alone sets `CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1`.
    - Its table also shows both sides at the default sixteen units, for information only.
      Those columns never fail the job.
      They show what a user's default release build gets.
    - Its harness, `examples/lex.rs`, splits each fixture into lines before both passes.
      The counted pass is lexing alone.
      Counts from before 2026-11-03 include that splitting and read higher.
    - Its counts are a review signal for the change itself.
      At the default sixteen units, an edit that only moves engine code between units shifted them by up to a few percent.
      On 2026-11-03, Python moved +0.6% and Go −6% on a change that measured ±0.05% at one unit.
      A gate that trips on layout alone gets ignored.
    - The wall-clock figures of rule 11 keep the default profile, because that layout is what users get.
13. A language on `crate::rules` marks `#[inline]` the dispatch its scan calls in another module.
    That covers the function that tries a pattern, and the matchers that table calls at every position.
    - The engine's methods on `Lexer<G>` are generic.
      Each language's copies are placed in one codegen unit.
      At the default sixteen units, a call from another module cannot be inlined unless the callee is `#[inline]`.
    - On 2026-11-03 that alone cost C 1.1% and the SQL port 7.6% more instructions at sixteen units than at one.
      Marking C's `match_pat` and SQL's `hit` and `pat_at` took both below their counts before the port.
    - The engine's own accessors were not the cause.
      Marking nine of them changed nothing.
    - The Callgrind job prints a note, never a failure, when a language's head runs more than 3% more instructions at sixteen units than at one.
      The note is the sign this rule was missed.
14. A port onto `crate::rules` passes on three conditions.
    - Identity is clean.
    - Its own instructions at one codegen unit (rule 12) are at parity with the lexer it replaces, or better.
    - Every other rule language stays within ±0.3% there.

    The pull request also records the instructions at sixteen units and the wall time in the default profile.
    They do not decide, because they follow layout across codegen units.
    Rule 13 is how a port closes that gap.
15. `mise run callgrind` counts every language with oracle fixtures.
    It reads the language list from the directories under `tests/oracle/`.
    A new language is counted without editing any list.
16. Instruction counts of one build are not exactly repeatable.
    The arena's map of interned strings keeps std's seeded hasher (lead, 2026-11-04).
    Its keys come from the text being lexed.
    The seeded hasher resists keys crafted to collide.
    Six runs of one build at one codegen unit on one box spread by these amounts (2026-11-04).
    The spread is the largest count minus the smallest, over the mean.
    - haskell and erb: 0.09%;
    - mermaid, html and julia: 0.06%;
    - nix: 0.04%;
    - html-derivative: 0.03%;
    - proto and markdown: 0.02%;
    - ruby: 0.01%;
    - python and c: under 0.01%;
    - go: none.

    The other engine languages intern through the same map and were not measured.
    A move of 0.1% or less in these languages is within that spread.
    The 0.5% gate sits five times above the largest.
17. Below its instruction table, `mise run callgrind` prints what one pass over each language's fixtures allocates on each side.
    It prints allocations, KiB and the most KiB one document held (`examples/memory.rs`).
    The counts are exact and never fail the job.
    Zero allocations on a second pass stays a test (`tests/it/alloc.rs`).
18. The wall-clock benchmarks and the comparison read `bench/corpus/`, never the oracle fixtures.
    Its README says what each tier stands for.
    A file enters only with its row in `bench/corpus.toml` and a permissive licence read at its source.
    `mise run check:corpus` holds the files and the rows together.
    Changing the corpus raises its `version`, and a record names the version it ran on.
    It also updates the `digest` line below `version`, which `check:corpus` prints when it differs.
    Two pull requests that raise `version` to the same number then conflict on `digest`.
    The second to land raises `version` again and takes the digest of the merged files.
19. The bench workspace (`Cargo.toml`, `src/`, `benches/`) is its own Cargo workspace, with its own `Cargo.lock`.
    The root workspace excludes it.
    `verify` does not build it.
    The Callgrind workflow's Bench job runs `mise run bench:check` on a pull request labelled `bench`.
    `bench:check` checks formatting, runs clippy with warnings denied, and runs each benchmark once, untimed.
    The comparison workspace, `compare/`, is its own workspace as well.
    `bench:check` checks its formatting and runs clippy on it without giallo.
    It never runs the comparison.
    Label a pull request that changes the public API, so the benchmarks keep building.
20. A Callgrind A/B on a box runs through `MISE_ENV=dev,bench mise run callgrind`.
    - That loads valgrind 3.27.1, the version `mise.bench.toml` pins, as CI does.
    - Run it as written, with mise's task auto-install on.
      Auto-install puts every tool the task reads on `PATH`: valgrind from `mise.bench.toml`, and taplo from `mise.toml`.
      A fresh box has neither when it sets `MISE_TASK_RUN_AUTO_INSTALL=false` or runs `bash mise-tasks/callgrind.sh`.
    - `callgrind.sh` stops when another valgrind is on `PATH`.
    - It builds the head with `--features textmate`, and the base too when the base has that feature.
    - `lib/commit-target.sh` sets `MBX_DISABLE=1`, so mbx cannot change a build's inlining.
    - Before trusting the rows, build each side three times and compare the sha256 of the binaries.
      A mismatch means the build is not repeatable, and the rows do not count.
    - Rows from any other setup do not count.
      On 2026-11-04 a box run outside the task, on valgrind 3.22, read SQL's base at 99.4 million instructions.
      CI read 94.2 million.
    - A row a decision relies on names the valgrind version it ran on.
