# Tasks

Each slice is one pull request in one `gh stack`.
Crate slices run the oracle, Identity and Callgrind workflows; none may change output.

## 1. Plan

- [ ] 1.1 The lead agrees the design, and the operator approves the public API → verify: their replies recorded here
- [x] 1.2 The lead approves `@shikijs/stream@4.6.0` as a bench-only dependency, or picks the alternative of driving Shiki's `codeToTokens` with `grammarState` directly → verify: the reply recorded here

  2026-11-05 UTC: the operator allowed `@shikijs/stream@4.6.0` in `bench/compare` only, pinned, with its licence in `bench/compare/README.md` (relayed by the lead).

  The operator has not approved the `Document` API yet (task 1.1).
  They asked whether a later line can change an earlier line's meaning.
  The design's section "A later line never changes an earlier one" answers it.
  The operator also asked that `Document` return styled tokens through the throughput change's per-stack style cache, with the surface agreed with perf-34.
  perf-34 agreed the surface on 2026-11-05, with two amendments.
  `cached_style` takes the theme, and a binding receives style-table entries as batches first use them.
  It is design decision 12.

## 2. Traces

- [ ] 2.1 `bench/traces.ts` writes the reply, tool-write and edit traces into `bench/traces/`, each naming its source, commit and licence → verify: running it twice gives the same bytes
- [ ] 2.2 A trace check fails when a trace and its source disagree, and runs in `check:local` → verify: it fails on an edited trace and passes on the written ones

## 3. `Document`: appends

- [ ] 3.1 `Document` with `push`, `push_str`, `finish`, `lex`, `take_changes`, `line` and `LineKind` → verify: a test streams every oracle fixture in token-sized deltas and in random byte splits, including splits inside characters, and finds every line's tokens equal to one-shot lexing
- [ ] 3.2 A completed line is lexed once, and a line unchanged since its provisional lexing is not lexed again → verify: a test counts `Highlighter::line` calls per streamed fixture
- [ ] 3.3 `Tail` and `set_tail` → verify: a test streams a 100 KiB line in four-byte deltas and bounds the bytes lexed
- [ ] 3.4 `Document` owns its theme, `lex` fills the per-stack style cache, and `DocumentLine::styled` reads it (design decision 12, after the cache lands) → verify: a test streams every oracle fixture with every oracle theme, switches the theme mid-stream, and finds every style equal to `Theme::style`
- [ ] 3.5 The crate docs gain a Streaming section with a doctest, and the README its one line → verify: the doctests pass
- [ ] 3.6 A changeset, `nishiki: patch` → verify: verctl lists it

## 4. `Document`: edits

- [ ] 4.1 `replace`, `EditError` and the stop where states agree → verify: a test applies random edits to every oracle fixture and finds the tokens equal to one-shot lexing of the result
- [ ] 4.2 The edit stop is counted → verify: a test replays the edit traces and finds the lines lexed past each hunk within the probe's figures
- [ ] 4.3 `reset` → verify: a test fills the arena to a small node limit, resets, and finds the tokens equal to one-shot lexing

## 5. Stream benchmarks

- [ ] 5.1 `bench/src/bin/stream.rs` times nishiki on every trace, and `mise run bench:stream` runs it → verify: `bench:check` runs it once
- [ ] 5.2 The harness counts how often a tail lexed again rebuilds its frames instead of taking the last line's over (design decision 11, need 5) → verify: the count is recorded here, and sent to perf-34 if it shows in the per-delta time
- [ ] 5.3 `bench/compare/shiki.ts stream` times Shiki and Pierre on the same traces → verify: a local run prints every engine's rows, with Shiki's fidelity at 100%
- [ ] 5.4 A recorded run on the reference host class, rendered by `bench:report` → verify: `check:bench-report` passes on the record

## 6. Hand-offs

- [ ] 6.1 Design decision 11's five needs are sent to perf-34 for the throughput change → verify: perf-34's reply, and the tasks they added, recorded here

  2026-11-05 UTC: sent with the figures of the research note, section 4.
  perf-34 replied that the throughput change takes needs 1 to 4:
  - first contact, through the candidate tables built from bit sets, then per-language slices for shell's `rule_info` and C's list tables, checked with `bench:cold` and `bench:latency` before and after each slice;
  - the cold first document, through the same slices, with the large tier rerun;
  - the `textmate` tier in `bench:latency` and `bench:cold`, as a bench slice;
  - a probe that counts the instructions of each prefix length of one long line under Callgrind, and fails on superlinear growth.

  Need 5, the resume frames of a tail lexed again, stays with this change, counted on the stream traces in task 5.2.
  Not yet verified: the tasks are not in the throughput change's `tasks.md` yet.

## 7. Archive

- [ ] 7.1 `openspec archive streaming -y`, with the specs' Purpose written → verify: `mise run specs` passes
