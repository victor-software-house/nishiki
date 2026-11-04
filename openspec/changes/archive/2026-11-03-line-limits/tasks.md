# Tasks

## 1. Decision

- [x] 1.1 Decide how nesting deeper than 32 works (design, section 4) → verify: the choice is recorded in `design.md`
  - 2026-11-02 UTC: the operator left it to best practice; `design.md` section 4 records the persistent-stack handle.
- [x] 1.2 Move every lexer onto one shared state and scope arena → verify: the oracle test passes for every language
  - 2026-11-02 UTC: Rust moved (`src/arena.rs`); its oracle fixtures still match with every line complete. Markdown, TypeScript and shell follow as they land.
  - 2026-11-03 UTC: every lexer takes the shared arena through [`Context`](https://github.com/victor-software-house/nishiki-archive/blob/fc6d39a0bb1df915e26d66243810318470602c82/src/language.rs), and the oracle test passes for every language in [CI run 41674564525](https://github.com/victor-software-house/nishiki-archive/actions/runs/41674564525).

## 2. Limits

- [x] 2.1 Add `Limits`, `Budget` and `LineReport`, and `Highlighter::new_with_limits` for Rust → verify: unit tests cover each limit reached and not reached
  - 2026-11-02 UTC: `tests/it/limits.rs` reaches each of the four limits and checks the tokens, report and next state; `src/limits.rs` tests the report, the saturating step count and `for_budget`. Passed on the operator's Mac; CI pending.
- [x] 2.2 Count work in the Rust lexer's main loop and lookahead scans → verify: generated hostile lines (the comparison's section 7.1 cases for Rust) stay linear and report the work limit when lowered
  - 2026-11-02 UTC: 100,000 repetitions of `/*`, `f(` and `a::<` finish within the default work limit. The work limit first caught `a::<`: the turbofish rule scanned to the line end for `>(` on every `::<`, so the lexer now finds the line's last `>(` once and compares.
- [x] 2.3 Apply the same limits to every other lexer, embedded lexers included → verify: each lexer's hostile-line tests pass
  - 2026-11-03 UTC: [`tests/it/limits/`](https://github.com/victor-software-house/nishiki-archive/tree/fc6d39a0bb1df915e26d66243810318470602c82/tests/it/limits) holds hostile-line tests for all 41 languages on main; they pass in [CI run 41674564525](https://github.com/victor-software-house/nishiki-archive/actions/runs/41674564525).
- [x] 2.4 Default limits → verify: the oracle test reports no limit on any fixture
  - 2026-11-03 UTC: [`tests/it/oracle.rs`](https://github.com/victor-software-house/nishiki-archive/blob/fc6d39a0bb1df915e26d66243810318470602c82/tests/it/oracle.rs#L169-L173) asserts every fixture line completes under the default limits; it passes in [CI run 41674564525](https://github.com/victor-software-house/nishiki-archive/actions/runs/41674564525).
