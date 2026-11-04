# Proposal

## Why

nishiki's benchmarks cannot back a claim about its speed.

- The cold-start figure is not cold.
  [`examples/bench.rs`][bench-rs] measures it in the same process, after 22 passes over the same document.
- Each per-line time is one sample, read with a timer whose overhead is the size of the fastest lines.
  One worst Markdown line read 347.9, 68.9 and 43.1 µs in three runs of the [first record][record].
- The per-language headline in that record is the slowest fixture.
  For C and C# that is Shiki's sample, a file under 1 KB, and each pass over it includes creating a highlighter.
- The corpus is the oracle fixtures, which were chosen for fidelity and grow with it.
- The recorded host shares cores: shell's slowest fixture ran at 11.6 to 37.3 MiB/s across three runs.
- The callgrind gate, the one part that works, counts 27 of the 46 languages with fixtures.
  Rust, TypeScript, shell, JSON, YAML, XML and HTML are not among them.
- There is no figure for memory, for the worst a line can cost, or for any other highlighter.

The [survey][survey] found no highlighter that measures all of this well.
nishiki can, because its callgrind gate is already ahead of the field.

## What Changes

- Each benchmark answers one named question, with its unit, its statistic and where it is gated.
- A benchmark corpus of its own: pinned, licensed, in size tiers per language, and stable when the oracle grows.
- A `bench/` workspace replaces `examples/bench.rs`:
  - Criterion for throughput and theme styling;
  - a latency program that repeats passes and removes timer overhead;
  - a cold-start program run as fresh processes;
  - a memory report.
- The callgrind gate keeps every guarantee it has and covers every language with fixtures.
  The list comes from the fixture directories, not a hand-kept line.
- Wall time never gates CI.
  A recorded run comes from a host class whose noise was measured, and a run whose anchor drifts is rejected.
- Results are JSON with a rendered Markdown summary, one pair per recorded run.
- A head-to-head comparison with syntect, tree-sitter-highlight, Shiki and others, in a separate workspace.
  It uses the same inputs and theme, keeps setup apart, and reports fidelity beside speed.
- `bench/` gets a README and a rewritten `AGENTS.md`.

## Capabilities

### New Capabilities

- `benchmarks`: what nishiki measures about its own speed and memory, how each figure is produced, where it gates, and how it is compared with other highlighters.

### Modified Capabilities

None.

## Impact

- `examples/bench.rs` and `mise-tasks/bench.sh` are replaced; `examples/lex.rs` stays where it is.
- `mise-tasks/callgrind.sh` and `.github/workflows/callgrind.yml` change their language list and run languages in parallel.
- New directories: `bench/corpus/`, the `bench/` Cargo workspace, and `bench/compare/`.
- New dependencies, each only in a bench workspace and never in the published crate.
  The operator approved each on 2026-11-03 (design, decision 14).
- No change to the crate's API or output, and no changeset.

[bench-rs]: https://github.com/victor-software-house/nishiki-archive/blob/fc6d39a0bb1df915e26d66243810318470602c82/examples/bench.rs
[record]: https://github.com/victor-software-house/nishiki-archive/blob/fc6d39a0bb1df915e26d66243810318470602c82/bench/results/namespace-4x8/2026-11-03-574fad99.md
[survey]: ../../../research/2026-11-03-benchmark-practice.md
