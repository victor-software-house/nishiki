# bench

Rules for measuring nishiki, beside the root guide's invariants.
`examples/bench.rs` is the harness; `mise run bench` runs it in release mode on every oracle fixture.
`targets.toml` holds what results are judged against; `results/` holds recorded runs.

1. A local run on a developer's machine is for comparing two commits of one change on that machine, and is never recorded.
   A recorded run, and any number a decision or a pull request relies on, comes from a quiet Linux x64 box, the cheapest that fits, created for the run and deleted by the run's own script, pass or fail.
2. Before a paid run, estimate its minutes and cost; a run expected to cost more than about $15 waits for the operator's approval.
3. A recorded run goes in `results/<host class>/<id>.md`, with the raw harness output unchanged beside it as `<id>.txt`.
   It states the commit, the host (CPU, cores, memory, OS), the toolchain, when it ran, the number of runs, and its minutes and cost.
4. Keep the commit a record cites reachable: push the tag `bench/<full sha>` when the record is committed, because a squash or a restack leaves it on no branch.
5. A claim that a change is faster shows the table before and after, from one host and one session of runs, with at least three runs each and the median reported.
   Build the before and after binaries each in its own clean target directory: a tree from `git archive` keeps the commit's file times, so cargo can reuse an older build and compare a stale binary (2026-11-02: a stale before-build turned a 1.05 gain into a reported 1.3).
   A sanity row helps: the before side must reproduce the last known figure for its commit.
6. A result regresses when the statistic its target names is worse than the last recorded result on the same host class by more than `tolerance_pct`, and by at least the unit's floor.
   A regression blocks the change unless its pull request states why it is accepted.
7. A target in `targets.toml` changes only by a dated decision, which the comment beside it cites; a target not set yet is listed with the reason.
8. Every comparison with another engine (syntect, Pierre's lexers, a TextMate interpreter) runs in a separate program under `research/`, never as a dependency of the crate.
9. A speed change never changes output: the oracle, colour, fuzz and limit tests stay green, and the run's report says so.
10. Termination on hostile input is a test (`tests/it/limits/<language>.rs`), not a benchmark: each language's generated hostile lines must finish within the default limits.
11. `mise run bench` builds with Cargo's default release profile on purpose.
    A downstream binary builds nishiki that way, so the figures are what it sees.
    Do not add LTO or `codegen-units` to the root profile to improve them.
12. `mise run callgrind`, which the Callgrind workflow runs on a pull request labelled `bench`, gates on instructions with both sides built at one codegen unit (`CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1`, set by the task only).
    Its table also shows both sides at the default sixteen units, for information only: those columns never fail the job, and they show what a user's default release build gets.
    Its counts are a review signal for the change itself: at the default sixteen units, an edit that only moves engine code between units shifted them by up to a few percent (2026-11-03: Python +0.6% and Go −6% on a change that measured ±0.05% at one unit), and a gate that trips on layout alone gets ignored.
    The wall-clock figures of rule 11 keep the default profile, because that layout is what users get.
13. A language on `crate::rules` marks `#[inline]` the dispatch its scan calls in another module: the function that tries a pattern, and the matchers that table calls at every position.
    The engine's methods on `Lexer<G>` are generic, so each language's copies are placed in one codegen unit, and at the default sixteen units a call from another module cannot be inlined unless the callee is `#[inline]`.
    On 2026-11-03 that alone cost C 1.1% and the SQL port 7.6% more instructions at sixteen units than at one; marking C's `match_pat` and SQL's `hit` and `pat_at` took both below their counts before the port.
    The engine's own accessors were not the cause: marking nine of them changed nothing.
    The Callgrind job prints a note, never a failure, when a language's head runs more than 3% more instructions at sixteen units than at one: the sign this rule was missed.
14. A port onto `crate::rules` passes when identity is clean, its own instructions at one codegen unit (rule 12) are at parity with the lexer it replaces or better, and every other rule language stays within ±0.3% there.
    The pull request also records the instructions at sixteen units and the wall time in the default profile, but they do not decide: they follow layout across codegen units, and rule 13 is how a port closes that gap.
