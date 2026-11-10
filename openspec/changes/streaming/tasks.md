# Tasks

Each slice is one pull request in one `gh stack`.
Crate slices run the oracle, Identity and Callgrind workflows; none may change output.
The styled tokens are in the [`document-styles` change][styles].
The traces, the stream benchmarks and the hand-offs are in the [`stream-benchmarks` change][bench].

## 1. Plan

- [x] 1.1 The lead agrees the design, and the operator approves the public API → verify: their replies recorded here

  2026-11-05 UTC: the lead agreed the design.
  The operator approved the `Document` API with the style surface of the `document-styles` change's decision 1 (relayed by the lead).

  2026-11-05 UTC, recorded first under the bench dependency's task: the operator had not approved the `Document` API yet.
  The operator asked whether a later line can change an earlier line's meaning.
  The design's section "A later line never changes an earlier one" answers it.

## 2. `Document`: appends

- [x] 2.1 `Document` with `push`, `push_str`, `finish`, `lex`, `take_changes`, `line` and `Freshness`.
  → verify: a test streams every oracle fixture in token-sized deltas and in random byte splits, including splits inside characters.
  It finds every line's tokens equal to one-shot lexing.

  2026-11-05 UTC: in [#591].
  `every_language_streams_as_shiki_lexes` in `tests/it/stream.rs` streams every fixture in token-sized deltas, in random byte splits and deferred, and passes in CI.
- [x] 2.2 A completed line is lexed once, and a line unchanged since its provisional lexing is not lexed again.
  → verify: a test counts `Highlighter::line` calls per streamed fixture

  2026-11-05 UTC: in [#591].
  The unit tests in `src/document/tests.rs` count the lexes of completed and last lines, and pass in CI.
- [x] 2.3 `Tail` and `set_tail` → verify: a test streams a 100 KiB line in four-byte deltas and bounds the bytes lexed

  2026-11-05 UTC: in [#592].
  `a_long_line_is_lexed_in_bounded_steps` streams a 100 KiB JSON line in four-byte deltas and bounds the bytes lexed.
- [x] 2.4 The crate docs gain a Streaming section with a doctest, and the README its one line → verify: the doctests pass

  2026-11-05 UTC: in [#591].
  The crate docs' Streaming section holds a doctest, and the README its line; the doctests pass in CI.
- [x] 2.5 A changeset, `nishiki: patch` → verify: verctl lists it

  2026-11-05 UTC: `mise run ver -- status` lists `document-appends.md`, `document-tail.md`, `document-edits.md` and `document-styled.md`, each `nishiki: patch`.

## 3. `Document`: edits

- [x] 3.1 `replace`, `EditError` and the stop where states match.
  → verify: a test applies random edits to every oracle fixture and finds the tokens equal to one-shot lexing of the result

  2026-11-05 UTC: in [#593].
  `every_language_edits_as_shiki_lexes` applies twelve random edits to every fixture, and undoes them, against one-shot lexing.
- [x] 3.2 The edit stop is counted.
  → verify: a test replays the edit traces and finds the lines lexed past each hunk within the probe's figures

  2026-11-05 UTC: in [#593].
  `an_edit_stops_where_the_states_agree` replays the edit traces; no hunk lexes more than two lines past its own.
- [x] 3.3 `reset` → verify: a test fills the arena to a small node limit, resets, and finds the tokens equal to one-shot lexing

  2026-11-05 UTC: in [#593].
  `a_reset_lexes_again_from_a_fresh_highlighter` fills a 64-node arena, resets, and finds the tokens of a fresh highlighter.

## 4. Archive

- [ ] 4.1 `openspec archive streaming -y`, with the specs' Purpose written → verify: `mise run specs` passes

[styles]: ../document-styles/tasks.md
[bench]: ../stream-benchmarks/tasks.md
[#591]: https://github.com/victor-software-house/nishiki-archive/pull/591
[#592]: https://github.com/victor-software-house/nishiki-archive/pull/592
[#593]: https://github.com/victor-software-house/nishiki-archive/pull/593
