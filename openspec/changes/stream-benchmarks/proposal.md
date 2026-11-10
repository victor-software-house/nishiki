# Proposal

## Motivation

The [`streaming` change][streaming] adds `Document`, for model replies, tool-written files and edits.
nishiki's per-line steady cost is 0.1 to 3 µs at p50 ([box run][latency]), so the document layer decides the per-delta cost.
That cost can be compared only on the deltas a real stream sends, against the engines callers would use instead.

Shiki ships streaming as [`@shikijs/stream`][shiki-stream], and Pierre as `StreamTokenizer` and `LiveTokenizer`.

Streaming also needs work from the lexers that the `streaming` change does not own.

This change was part of the `streaming` change until that change was split to fit the OpenSpec word budgets.

## Changes

- Bench traces of model replies, tool-written files and edit sequences, in `bench/traces/`, each tied to its licensed source.
- A harness that times nishiki, Shiki and Pierre on the traces, each driven through its users' API.
  It reports setup, first-delta latency, steady per-delta figures and final fidelity to Shiki.
- What streaming needs from the lexers goes to the [throughput change][throughput] as hand-offs.
  The resume frames of a tail lexed again stay here, counted on the traces.

The design is in [design.md](design.md).
The research note behind it is [2026-11-05-streaming][note].

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- [`benchmarks`][benchmarks]: per-delta latency and final fidelity on delta traces, for nishiki, Shiki and Pierre.

## Impact

- New: `bench/traces/`, `bench/traces.ts`, `bench/src/bin/stream.rs`, a stream mode in `bench/compare/shiki.ts`.
- The comparison workspace pins [`@shikijs/stream@4.6.0`][shiki-stream] (MIT), to drive Shiki as its users do.
  The operator allowed it in `bench/compare` only, with its licence in that workspace's README (2026-11-05).
- No crate dependency is added, and the crate's API does not change.

[streaming]: ../streaming/proposal.md
[latency]: ../../research/2026-11-05-streaming/latency-cold-68620d9e.txt
[shiki-stream]: https://www.npmjs.com/package/@shikijs/stream/v/4.6.0
[throughput]: https://github.com/victor-software-house/nishiki-archive/blob/99416359c42a215e43d27db1e9a950fe2b21fe25/openspec/changes/throughput/proposal.md
[note]: ../../research/2026-11-05-streaming.md
[benchmarks]: ../../specs/benchmarks/spec.md
