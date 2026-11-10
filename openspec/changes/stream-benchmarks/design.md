# Design

## Context

This change measures the [`streaming` change][streaming]'s `Document` against Shiki and Pierre.
Its decisions were decisions 10 and 11 of that change's design until the change was split.

## Decisions

### 1. Benchmarks

Traces live in `bench/traces/`, written by `bench/traces.ts` from files with licences already read:

| Trace | Source | Deltas |
|:--|:--|:--|
| Model reply | the `reply`-tier corpus files | token-sized, as the probe cuts them |
| Tool-written file | medium-tier corpus files, one per language | token-sized, and the whole file in one delta |
| Edit sequence | nishiki's own history, `-U0` hunks | one `replace` per hunk |

- Each trace file names its source file and commit.
  `check:corpus` style checks hold the traces and their sources together.
- `bench/src/bin/stream.rs` times nishiki per delta: the mutation, `lex(Until::End)` with its styling, and reading the changed lines through `styled`.
- `bench/compare/shiki.ts stream` times Shiki with `ShikiStreamTokenizer`, and Pierre with `StreamTokenizer` for appends and `LiveTokenizer` for edits.
  Each runs as its users drive it.
- Every engine reports setup, first-delta latency, and steady per-delta p50, p99 and maximum.
  It also reports the final output's fidelity to Shiki on the whole text.
- Recorded runs follow [bench/AGENTS.md][bench-agents]: the reference host class, data first, never a CI gate.

### 2. Hand-offs to the throughput change

Streaming needs these from the lexers.
Each goes to the [throughput change][throughput] (owner perf-34), not into this one.

| Need | Today ([note][note], section 4) | Ask |
|:--|:--|:--|
| Cheap first contact for every language | first line 100 to 530 µs on the rule engine, 966 µs to 1.75 ms in shell | a first line under 100 µs, so a fence's first line lands in the frame it arrives in |
| A cold first document | 4.9 ms for shell's small file | the same cause as above, measured by `bench:cold` |
| The `textmate` tier measured | not built by the bench | Assembly and HAML in `bench:latency` and `bench:cold` |
| Line work linear in line length | not measured on long partial lines | a probe that lexes one long line at every prefix length, so the bound of the `streaming` design's decision 4 holds |
| The resume path on a line lexed again | the tail starts from the same state each delta, so the engines rebuild its frames | count it on the stream traces, and keep the frames when it shows |

## Risks / Trade-offs

- [The generated deltas are smaller than a real model's] → The flicker figures are an upper bound.
  The trace generator can take recorded transcripts later without changing the harness.
- [A bench dependency on `@shikijs/stream`] → It is MIT, pinned at 4.6.0 with Shiki.
  It sits in `bench/compare` only and is never shipped, as the operator allowed (2026-11-05).
  Its licence goes in that workspace's README.

[note]: ../../research/2026-11-05-streaming.md
[bench-agents]: https://github.com/victor-software-house/nishiki-archive/blob/68620d9ef4c7eb99a91c30f3a3d6788a2c4aaebf/bench/AGENTS.md
[throughput]: https://github.com/victor-software-house/nishiki-archive/blob/99416359c42a215e43d27db1e9a950fe2b21fe25/openspec/changes/throughput/proposal.md
[streaming]: ../streaming/design.md
