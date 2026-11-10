# Tasks

Each slice is one pull request in one `gh stack`.
These tasks were tasks 1.2, 2, 5 and 6 of the [`streaming` change][streaming] until that change was split.

## 1. Plan

- [x] 1.1 The lead approves `@shikijs/stream@4.6.0` as a bench-only dependency, or picks the alternative of driving Shiki's `codeToTokens` with `grammarState` directly → verify: the reply recorded here

  2026-11-05 UTC: the operator allowed `@shikijs/stream@4.6.0` in `bench/compare` only, pinned, with its licence in `bench/compare/README.md` (relayed by the lead).

## 2. Traces

- [x] 2.1 `bench/traces.ts` writes the reply, tool-write and edit traces into `bench/traces/`.
  Each trace names its source, commit and licence.
  → verify: running it twice gives the same bytes

  2026-11-05 UTC: in [#588].
  Locally, `bun bench/traces.ts write` run twice left `bench/traces/` unchanged, 22 traces.
- [x] 2.2 A trace check fails when a trace and its source differ, and runs in `check:local`.
  → verify: it fails on an edited trace and passes on the written ones

  2026-11-05 UTC: in [#588], and held to the source at the `to` commit in [#615].
  It runs in `check:local`.
  Locally, it failed on an edit trace whose last replacement and `sha256` were both changed, and passed on the written traces.

## 3. Stream benchmarks

- [x] 3.1 `bench/src/bin/stream.rs` times nishiki on every trace, and `mise run bench:stream` runs it → verify: `bench:check` runs it once

  2026-11-05 UTC: in [#619].
  On a box, the `bench:check` line replays one reply, one tool-write and one edit trace twice.
  All 22 traces replay exact with 100.00% fidelity.
- [ ] 3.2 The stream bench counts how often a tail lexed again rebuilds its frames, instead of taking over the last line's.
  This is design decision 2, need 5.
  → verify: the count is recorded here, and sent to perf-34 if it shows in the per-delta time.
- [x] 3.3 `bench/compare/shiki.ts stream` times Shiki and Pierre on the same traces → verify: a local run prints every engine's rows, with Shiki's fidelity at 100%

  2026-11-05 UTC: in [#620].
  Locally, the comparison printed rows for `shiki-stream`, `pierre-stream` and `pierre-live` on every trace, with `shiki-stream` at 100% fidelity.
- [ ] 3.4 A recorded run on the reference host class, rendered by `bench:report` → verify: `check:bench-report` passes on the record

## 4. Hand-offs

- [x] 4.1 Design decision 2's five needs are sent to perf-34 for the throughput change.
  → verify: perf-34's reply, and the tasks they added, recorded here

  2026-11-05 UTC: sent with the figures of section 4 of the research note.
  perf-34 replied that the throughput change takes needs 1 to 4:
  - first contact, through the candidate tables built from bit sets, then per-language slices for shell's `rule_info` and C's list tables.
    Each slice is checked with `bench:cold` and `bench:latency` before and after;
  - the cold first document, through the same slices, with the large tier rerun;
  - the `textmate` tier in `bench:latency` and `bench:cold`, as a bench slice;
  - a probe that counts the instructions of each prefix length of one long line under Callgrind, and fails on superlinear growth.

  Need 5, the resume frames of a tail lexed again, stays with this change, counted on the stream traces in task 3.2.
  Verified 2026-11-05 UTC: they are tasks 8.1 to 8.4 of the throughput change's `tasks.md`.

## 5. Archive

- [ ] 5.1 `openspec archive stream-benchmarks -y` → verify: `mise run specs` passes

[streaming]: ../streaming/tasks.md
[#588]: https://github.com/victor-software-house/nishiki-archive/pull/588
[#615]: https://github.com/victor-software-house/nishiki-archive/pull/615
[#619]: https://github.com/victor-software-house/nishiki-archive/pull/619
[#620]: https://github.com/victor-software-house/nishiki-archive/pull/620
