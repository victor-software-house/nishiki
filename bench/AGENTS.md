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
6. A result regresses when the statistic its target names is worse than the last recorded result on the same host class by more than `tolerance_pct`, and by at least the unit's floor.
   A regression blocks the change unless its pull request states why it is accepted.
7. A target in `targets.toml` changes only by a dated decision, which the comment beside it cites; a target not set yet is listed with the reason.
8. Every comparison with another engine (syntect, Pierre's lexers, a TextMate interpreter) runs in a separate program under `research/`, never as a dependency of the crate.
9. A speed change never changes output: the oracle, colour, fuzz and limit tests stay green, and the run's report says so.
10. Termination on hostile input is a test (`tests/it/limits.rs`), not a benchmark: each language's generated hostile lines must finish within the default limits.
