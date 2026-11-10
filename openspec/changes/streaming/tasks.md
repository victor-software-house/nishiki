# Tasks

Each slice is one pull request in one `gh stack`.
Crate slices run the oracle, Identity and Callgrind workflows; none may change output.

## 1. Plan

- [x] 1.1 The lead agrees the design, and the operator approves the public API → verify: their replies recorded here

  2026-11-05 UTC: the lead agreed the design.
  The operator approved the `Document` API with the style surface of decision 12 (relayed by the lead).
- [x] 1.2 The lead approves `@shikijs/stream@4.6.0` as a bench-only dependency, or picks the alternative of driving Shiki's `codeToTokens` with `grammarState` directly → verify: the reply recorded here

  2026-11-05 UTC: the operator allowed `@shikijs/stream@4.6.0` in `bench/compare` only, pinned, with its licence in `bench/compare/README.md` (relayed by the lead).

  The operator has not approved the `Document` API yet (task 1.1).
  The operator asked whether a later line can change an earlier line's meaning.
  The design's section "A later line never changes an earlier one" answers it.

  The operator also asked that `Document` return styled tokens through the throughput change's per-stack style cache.
  The surface was to be agreed with perf-34.
  perf-34 agreed the surface on 2026-11-05, with two amendments.
  `cached_style` takes the theme, and a binding receives style-table entries as batches first use them.
  It is design decision 12.

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

## 3. `Document`: appends

- [x] 3.1 `Document` with `push`, `push_str`, `finish`, `lex`, `take_changes`, `line` and `Freshness`.
  → verify: a test streams every oracle fixture in token-sized deltas and in random byte splits, including splits inside characters.
  It finds every line's tokens equal to one-shot lexing.

  2026-11-05 UTC: in [#591].
  `every_language_streams_as_shiki_lexes` in `tests/it/stream.rs` streams every fixture in token-sized deltas, in random byte splits and deferred, and passes in CI.
- [x] 3.2 A completed line is lexed once, and a line unchanged since its provisional lexing is not lexed again.
  → verify: a test counts `Highlighter::line` calls per streamed fixture

  2026-11-05 UTC: in [#591].
  The unit tests in `src/document/tests.rs` count the lexes of completed and last lines, and pass in CI.
- [x] 3.3 `Tail` and `set_tail` → verify: a test streams a 100 KiB line in four-byte deltas and bounds the bytes lexed

  2026-11-05 UTC: in [#592].
  `a_long_line_is_lexed_in_bounded_steps` streams a 100 KiB JSON line in four-byte deltas and bounds the bytes lexed.
- [x] 3.4 `Document` owns its theme, `lex` fills the per-stack style cache, and `DocumentLine::styled` reads it (design decision 12, after the cache lands).
  → verify: a test streams every oracle fixture with every oracle theme and switches the theme mid-stream.
  It finds every style equal to `Theme::style`.

  2026-11-05 UTC: in the pull request of `feat/document-styled`.
  `every_language_streams_styled_as_the_theme_styles` streams every fixture under the three oracle themes.
  It switches the theme at the middle delta and again after the end, and finds every style equal to `Theme::style`.
- [x] 3.5 The crate docs gain a Streaming section with a doctest, and the README its one line → verify: the doctests pass

  2026-11-05 UTC: in [#591].
  The crate docs' Streaming section holds a doctest, and the README its line; the doctests pass in CI.
- [x] 3.6 A changeset, `nishiki: patch` → verify: verctl lists it

  2026-11-05 UTC: `mise run ver -- status` lists `document-appends.md`, `document-tail.md`, `document-edits.md` and `document-styled.md`, each `nishiki: patch`.

## 4. `Document`: edits

- [x] 4.1 `replace`, `EditError` and the stop where states match.
  → verify: a test applies random edits to every oracle fixture and finds the tokens equal to one-shot lexing of the result

  2026-11-05 UTC: in [#593].
  `every_language_edits_as_shiki_lexes` applies twelve random edits to every fixture, and undoes them, against one-shot lexing.
- [x] 4.2 The edit stop is counted.
  → verify: a test replays the edit traces and finds the lines lexed past each hunk within the probe's figures

  2026-11-05 UTC: in [#593].
  `an_edit_stops_where_the_states_agree` replays the edit traces; no hunk lexes more than two lines past its own.
- [x] 4.3 `reset` → verify: a test fills the arena to a small node limit, resets, and finds the tokens equal to one-shot lexing

  2026-11-05 UTC: in [#593].
  `a_reset_lexes_again_from_a_fresh_highlighter` fills a 64-node arena, resets, and finds the tokens of a fresh highlighter.

## 5. Stream benchmarks

- [x] 5.1 `bench/src/bin/stream.rs` times nishiki on every trace, and `mise run bench:stream` runs it → verify: `bench:check` runs it once

  2026-11-05 UTC: in [#619].
  On a box, the `bench:check` line replays one reply, one tool-write and one edit trace twice.
  All 22 traces replay exact with 100.00% fidelity.
- [ ] 5.2 The stream bench counts how often a tail lexed again rebuilds its frames, instead of taking over the last line's.
  This is design decision 11, need 5.
  → verify: the count is recorded here, and sent to perf-34 if it shows in the per-delta time.
- [x] 5.3 `bench/compare/shiki.ts stream` times Shiki and Pierre on the same traces → verify: a local run prints every engine's rows, with Shiki's fidelity at 100%

  2026-11-05 UTC: in [#620].
  Locally, the comparison printed rows for `shiki-stream`, `pierre-stream` and `pierre-live` on every trace, with `shiki-stream` at 100% fidelity.
- [ ] 5.4 A recorded run on the reference host class, rendered by `bench:report` → verify: `check:bench-report` passes on the record

## 6. Hand-offs

- [x] 6.1 Design decision 11's five needs are sent to perf-34 for the throughput change.
  → verify: perf-34's reply, and the tasks they added, recorded here

  2026-11-05 UTC: sent with the figures of section 4 of the research note.
  perf-34 replied that the throughput change takes needs 1 to 4:
  - first contact, through the candidate tables built from bit sets, then per-language slices for shell's `rule_info` and C's list tables.
    Each slice is checked with `bench:cold` and `bench:latency` before and after;
  - the cold first document, through the same slices, with the large tier rerun;
  - the `textmate` tier in `bench:latency` and `bench:cold`, as a bench slice;
  - a probe that counts the instructions of each prefix length of one long line under Callgrind, and fails on superlinear growth.

  Need 5, the resume frames of a tail lexed again, stays with this change, counted on the stream traces in task 5.2.
  Verified 2026-11-05 UTC: they are tasks 8.1 to 8.4 of the throughput change's `tasks.md`.

## 7. Archive

- [ ] 7.1 `openspec archive streaming -y`, with the specs' Purpose written → verify: `mise run specs` passes

[#588]: https://github.com/victor-software-house/nishiki-archive/pull/588
[#591]: https://github.com/victor-software-house/nishiki-archive/pull/591
[#592]: https://github.com/victor-software-house/nishiki-archive/pull/592
[#593]: https://github.com/victor-software-house/nishiki-archive/pull/593
[#615]: https://github.com/victor-software-house/nishiki-archive/pull/615
[#619]: https://github.com/victor-software-house/nishiki-archive/pull/619
[#620]: https://github.com/victor-software-house/nishiki-archive/pull/620
