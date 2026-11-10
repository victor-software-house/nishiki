# Tasks

These tasks were tasks 1, 2, 3 and 5.2 of the [`engine-handoff` change][handoff] until that change was split.

Tasks 1.5, 2, 3 and 4 are deferred.
`Halts`, `Grammar::rule_halts` and entries replace the two halt arguments in a cross-language PR of their own.
The engine's logic does not change when they do.

## 1. Agreement

- [x] 1.1 Halt semantics agreed with xml-html (ERB's halt at any depth, Vue's entries) → verify: their reply recorded in design section 10
  - xml-html, 2026-11-03:
    - `any` has injection priority and skips comments;
    - the full stack at an `any` halt, and halts tried at place 0;
    - TypeScript's `expression` as the one entry;
    - html-derivative a language of its own;
    - a covered second hand-off dropped;
    - mixed repository rules compiled into the engine's table;
    - ERB moves onto `Halts` in its second PR.
- [x] 1.2 Engine profile agreed with engine-spans (cost of a `&dyn` halt per place) → verify: their reply recorded in design section 10
  - engine-spans, 2026-11-03: `&dyn Fn` is fine; keep the `halt: None` branch; `#[inline]` dispatch; halts stay pure; label the PR `bench`.
- [x] 1.3 GraphQL port's embedding agreed with primitives → verify: their reply recorded in design section 10
  - primitives, 2026-11-03: `lex_embedding_until` maps onto `Halts`; nested hand-offs forward only the outer `any`; halts per rule from the grammar.
- [x] 1.4 [#159][pr159]'s dispatch agreed with go-lexer → verify: their reply recorded in design section 10
  - go-lexer, 2026-11-03:
    - halts on the `Lexer`;
    - `any` only in the halting instance;
    - SQL, C, shell and GraphQL split too;
    - Ruby's heredocs kept;
    - `#[inline]` dispatch;
    - the shared budget waits on core-fix.
- [ ] 1.5 Operator approves the design → verify: approval cited here

## 2. One halt contract

- [ ] 2.1 `Halts` and `Language::lex_until(…, &Halts)` on top of [#159][pr159] and the GraphQL port.
  The rule engine and TypeScript honour `before`, `after` and `any`.
  → verify: HTML, GraphQL and Ruby heredoc fixtures unchanged.
  A test per test kind checks the halt's place.
- [ ] 2.2 The comment skip and the full state at an `any` halt.
  → verify: a test halts inside a JavaScript string and resumes from the returned state.
- [ ] 2.3 Halts tried at place 0 on a blank line → verify: a test with `^(\s*)(?=…)` as `any`
- [ ] 2.4 `Grammar::rule_halts`, replacing [#159][pr159]'s single `end` and `after` pair → verify: GraphQL and Ruby heredoc fixtures unchanged
- [ ] 2.5 The same split for SQL's and C's scans and for shell's and GraphQL's engines.
  → verify: Callgrind at one codegen unit keeps each under 1.005 of its base.
- [ ] 2.6 Ruby's `lex_until` on `&Halts` keeps its heredocs, through `lex_embedding_until` → verify: a Ruby region with `<<~SQL` embedded by ERB matches Shiki
- [ ] 2.7 Nested hand-offs forward the outer `any` → verify: ERB in a JavaScript string inside HTML halts JavaScript

## 3. Entries

- [ ] 3.1 `Entry` and `start_embedded(arena, base, entry)` → verify: every caller passes `Entry::Root` and no fixture changes

## 4. Follow-ups

- [ ] 4.1 ERB's any-depth injection (xml-html)

[pr159]: https://github.com/victor-software-house/nishiki-archive/pull/159
[handoff]: ../engine-handoff/tasks.md
