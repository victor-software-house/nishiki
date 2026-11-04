# Benchmarks

How nishiki measures its speed and memory.
The rules for measuring are in [AGENTS.md](AGENTS.md); the reasons are in the `bench-methodology` change under `openspec/changes/`.

## What each figure answers

| Question | Figure | Run with |
|:--|:--|:--|
| How fast does nishiki lex a document? | MiB/s per language and tier | `mise run bench lex/` |
| What does creating a highlighter cost? | time per `Highlighter::new` | `mise run bench new/` |
| How fast does it style tokens? | time per token, github-dark | `mise run bench style/` |
| How fast does it colour a document? | MiB/s from text to colours | `mise run bench colour/` |
| Did a change make lexing slower? | instructions, head over base | `mise run callgrind`, in CI on a pull request labelled `bench` |
| How much memory does a document take? | allocations, bytes and peak | printed below the callgrind table |

Per-line latency, cold start, the worst line under the default limits and the comparison with other highlighters come in later steps of the change.
Until then, `examples/bench.rs` still gives per-line latency and a first-line time.

## Where things are

- [corpus/](corpus/README.md): the files the wall-clock benchmarks read, and [corpus.toml](corpus.toml), which lists them.
- `Cargo.toml`, `src/` and `benches/`: the bench workspace, apart from the crate's, with Criterion.
- [targets.toml](targets.toml): what a recorded run is judged against.
- `results/`: recorded runs, one folder per host class.

## Running

`mise run bench` runs every benchmark; an argument filters them by name.
Criterion's `--save-baseline <name>` and `--baseline <name>` compare two runs on one machine.
A figure from a laptop compares two commits on that laptop and nothing else; recorded figures come from the host class AGENTS.md names.
