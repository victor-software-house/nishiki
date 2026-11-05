# Benchmarks

How nishiki measures its speed and memory.
The rules for measuring are in [AGENTS.md](AGENTS.md).
The reasons are in the `bench-methodology` change under `openspec/changes/`.

## What each figure answers

| Question | Figure | Run with |
|:--|:--|:--|
| How fast does nishiki lex a document? | MiB/s per language and tier | `mise run bench lex/` |
| What does creating a highlighter cost? | time per `Highlighter::new` | `mise run bench new/` |
| How fast does it style tokens? | time per token, github-dark | `mise run bench style/` |
| How fast does it colour a document? | MiB/s from text to colours | `mise run bench colour/` |
| What does one streamed line cost? | µs per line: steady p50, p99 and maximum, and the first-contact maximum | `mise run bench:latency` |
| How soon is the first line coloured? | µs in a fresh process to create a highlighter, lex the first line and the whole file; theme loading | `mise run bench:cold` |
| Did a change make lexing slower? | instructions, head over base | `mise run callgrind`, in CI on a pull request labelled `bench` |
| How much memory does a document take? | allocations, bytes and peak | printed below the callgrind table |

The worst line under the default limits and the comparison with other highlighters come in later steps of the change.

## Where things are

- [corpus/](corpus/README.md): the files the wall-clock benchmarks read, and [corpus.toml](corpus.toml), which lists them.
- [traces/](traces/README.md): the delta traces the stream benchmarks replay, written by [traces.ts](traces.ts).
- `Cargo.toml`, `src/` and `benches/`: the bench workspace, apart from the crate's, with Criterion.
- [targets.toml](targets.toml): what a recorded run is judged against.
  Its throughput floors come from the first record, at main `0a58343a`.
  That record predates the speed work of 2026-11-04.
  The floors catch a regression; they are not targets.
  The next reference record raises them.
  First-contact and first-line times have no floor until a record shows them stable.
- `results/`: recorded runs, one folder per host class.

## The newest record

`mise run bench:record` records a session as `results/namespace-8x16/<date>-<sha8>.json`.
It runs only on that reference class ([why](../openspec/research/2026-11-04-bench-host-probe-criterion.md)).
`mise run bench:report` renders each record's page and the summary below.
`check:bench-report` fails when either differs from its JSON.

<!-- record summary: rendered by `mise run bench:report summary` -->

namespace-8x16, 2026-11-04, commit `0a58343a`; [the full record](results/namespace-8x16/2026-11-04-0a58343a.md).
These figures are indicative only.
No host class keeps two runs of one build within 2%, and this one moves by up to 11% ([probe](../openspec/research/2026-11-04-bench-host-probe-criterion.md)).
Speed decisions rest on Callgrind instruction counts.

| Figure | Median over languages | Worst | Median spread over rounds |
|:--|--:|--:|--:|
| Medium throughput, MiB/s | 61.8 | 8.1 (cpp) | 2.4% |
| Medium steady p99 per line, µs | 2.4 | 14.5 (haskell) | 4.7% |
| Medium first-contact max per line, µs | 68.6 | 19,646.3 (cmake) | 29.8% |
| Small first line in a fresh process, µs | 129.9 | 18,395.3 (cmake) | per process |

<!-- end of record summary -->

## Running

`mise run bench` runs every benchmark.
An argument filters them by name.
Criterion's `--save-baseline <name>` and `--baseline <name>` compare two runs on one machine.
A figure from a laptop compares two commits on that laptop and nothing else.
Recorded figures come from the host class AGENTS.md names.
