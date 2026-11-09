# Proposal

## Motivation

The operator's goal is the fastest highlighting for LLM streaming, with output that equals Shiki's.
The use cases are model replies streamed delta by delta, tool calls, and file edits and writes.
Code arrives in pieces and is rendered live.

nishiki already has the hard part.
[`Highlighter::line`][highlighter] lexes one line from a `State`, and a `State` is a `Copy` handle that compares in one step.
A completed line's tokens depend only on the state before it and its own text.
So a completed line is final, exactly as Shiki colours it.

What nishiki lacks is the document around the lines.
Every caller now rebuilds it:

- splitting deltas into lines, including a delta that ends inside a UTF-8 character;
- caching each line's tokens and end state;
- re-lexing only the line still being written;
- re-lexing after an edit until the states match again;
- telling the renderer which lines changed.

Shiki ships this as [`@shikijs/stream`][shiki-stream], and Pierre as `StreamTokenizer` and `LiveTokenizer`.
nishiki's per-line steady cost is 0.1 to 3 µs at p50 ([box run][latency]), so the document layer decides the per-delta cost.

## Changes

- A new public type, `Document`, owns one `Highlighter`, the text, and each line's tokens and end state.
  - `push` and `push_str` append a delta.
    A delta may end mid-line or mid-UTF-8 character.
  - `replace` swaps a byte range for new text, for edits and diffs.
  - `finish` ends the stream, so the last line becomes final.
  - `lex` lexes up to a line or to the end, so a caller can lex its viewport first.
  - `take_changes` reports the lines whose text or tokens changed since the last call.
  - `line` gives a line's text, tokens and kind: exact, provisional or stale.
  - Each line's tokens come styled with the document's theme, through the [throughput change][throughput]'s per-stack style cache.
- A completed line is lexed once.
  The line still being written is lexed again on each delta.
  Its tokens carry the offset where its last token starts, so a renderer can hold that token back.
- After an edit, lexing runs from the first changed line until a line's end state equals its cached one, then stops.
- Bench traces of model replies, tool-written files and edit sequences, and a harness that times nishiki, Shiki and Pierre on them.
- What streaming needs from the lexers goes to the [throughput change][throughput] as hand-offs.

The design, with its measurements, is in [design.md](design.md).
The research note behind it is [2026-11-05-streaming][note].

## Capabilities

### New Capabilities

- `streaming`: a document highlighted as it arrives and as it is edited, whose final tokens equal those of lexing the whole text line by line.

### Modified Capabilities

- [`benchmarks`][benchmarks]: per-delta latency and final fidelity on delta traces, for nishiki, Shiki and Pierre.

[`shiki-fidelity`][fidelity] holds as it is: a document's final tokens are the tokens `Highlighter::line` gives.

## Impact

- New: `src/document.rs`, `bench/traces/`, `bench/traces.ts`, `bench/src/bin/stream.rs`, a stream mode in `bench/compare/shiki.ts`.
- The public API gains `Document` and its types.
  The change is additive, so the release is a patch (invariant 6).
- No crate dependency is added.
- The comparison workspace would pin [`@shikijs/stream@4.6.0`][shiki-stream] (MIT), to drive Shiki as its users do.
  The operator allowed it in `bench/compare` only, with its licence in that workspace's README (2026-11-05).
- npm stays closed (invariant 8).
  The design notes what a later WebAssembly or N-API binding needs, and builds none of it.

[highlighter]: https://github.com/victor-software-house/nishiki-archive/blob/68620d9ef4c7eb99a91c30f3a3d6788a2c4aaebf/src/highlighter.rs#L67-L100
[shiki-stream]: https://www.npmjs.com/package/@shikijs/stream/v/4.6.0
[latency]: ../../research/2026-11-05-streaming/latency-cold-68620d9e.txt
[throughput]: https://github.com/victor-software-house/nishiki-archive/blob/99416359c42a215e43d27db1e9a950fe2b21fe25/openspec/changes/throughput/proposal.md
[note]: ../../research/2026-11-05-streaming.md
[fidelity]: ../../specs/shiki-fidelity/spec.md
[benchmarks]: ../../specs/benchmarks/spec.md
