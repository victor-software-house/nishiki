# Tasks

## 1. The module

- [x] 1.1 `nishiki::unstable` and its cfg setup, with the docs.rs settings and a CI build with the cfg; `Payload` becomes `CarriedState`.
  → verify: the docs.rs preview shows the module with its badge, and the semver check runs without the cfg
  - 2026-11-10: on a box, lint, tests, doctests and `check:unstable` pass.
    A nightly rustdoc build with both cfgs passes under `-D warnings`.
    Its module list shows the `nishiki_unstable` badge on `unstable`.
    The semver job sets no `RUSTFLAGS`, so it builds without the cfg.
- [ ] 1.2 `RuleSyntax<G>` under `nishiki::unstable`, so `Highlighter<S: Syntax>` runs a caller's grammar; its own pull request, with the `bench` label.
  → verify: a test highlights a line with a grammar the test defines, and Callgrind shows no language's count rising
  - 2026-11-10: on a box, lint, `check:features`, `check:unstable`, tests and doctests pass.
    `a_grammar_the_test_defines_highlights_lines` highlights two lines with a grammar the test defines.
    `a_language_highlighter_has_no_room_for_buffers` asserts that a `Language` highlighter keeps its size.
    The tick waits for Callgrind on the pull request.
