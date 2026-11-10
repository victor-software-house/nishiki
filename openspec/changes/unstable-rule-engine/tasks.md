# Tasks

## 1. The module

- [x] 1.1 `nishiki::unstable` and its cfg setup, with the docs.rs settings and a CI build with the cfg; `Payload` becomes `CarriedState`.
  → verify: the docs.rs preview shows the module with its badge, and the semver check runs without the cfg
  - 2026-11-10: on a box, lint, tests, doctests and `check:unstable` pass.
    A nightly rustdoc build with both cfgs passes under `-D warnings`.
    Its module list shows the `nishiki_unstable` badge on `unstable`.
    The semver job sets no `RUSTFLAGS`, so it builds without the cfg.
- [x] 1.2 `RuleSyntax<G>` under `nishiki::unstable`, so `Highlighter<S: Syntax>` runs a caller's grammar; its own pull request, with the `bench` label.
  → verify: a test highlights a line with a grammar the test defines, and Callgrind shows no language's count rising
  - 2026-11-10: on a box, lint, `check:features`, `check:unstable`, tests and doctests pass.
    `a_grammar_the_test_defines_highlights_lines` highlights two lines with a grammar the test defines.
    `a_language_highlighter_has_no_room_for_buffers` asserts that a `Language` highlighter keeps its size.
  - 2026-11-10: [#724][pr724] adds `RuleSyntax<G>`, and its Callgrind failed on JSON, JSONC, JSON Lines and HCL (design section 2.1).
    [#727][pr727] compiles it only with the cfg and in its test, and [its Callgrind run][run727] passes.
- [ ] 1.3 A build with the cfg calls the arena directly (design section 2.1); its own pull request, with the `bench` label.
  → verify: at one codegen unit, both builds make 550 direct calls to `Arena::child` and 196 to `Arena::push`, with every Callgrind row at or below 1.000
  - 2026-11-10: no code shape on stable rustc reaches parity ([research note][exports-note]).
    The task waits for stable rustc to relax ELF relocations by default, or to stabilize default symbol visibility.
    Default symbol visibility is tracked in [rust-lang/rust#131090][rust131090].
  - 2026-11-10: relaxed relocations have no tracking issue.
    [rust-lang/rust#106511][rust106511] made them the default, and [rust-lang/rust#115267][rust115267] reverted that four days later.
    [rust-lang/rust#163951][rust163951] documents the option as x86-only.

[exports-note]: ../../research/2026-11-10-rule-syntax-exports.md
[pr724]: https://github.com/victor-software-house/nishiki-archive/pull/724
[pr727]: https://github.com/victor-software-house/nishiki-archive/pull/727
[run727]: https://github.com/victor-software-house/nishiki-archive/actions/runs/42879287900/job/130050925736
[rust106511]: https://github.com/rust-lang/rust/pull/106511
[rust115267]: https://github.com/rust-lang/rust/pull/115267
[rust131090]: https://github.com/rust-lang/rust/issues/131090
[rust163951]: https://github.com/rust-lang/rust/pull/163951
