# Tasks

## 1. Agreement

- [x] 1.1 Halt semantics agreed with xml-html (ERB's halt at any depth, Vue's entries) → verify: their reply recorded in design section 9
  - xml-html, 2026-11-03: `any` has injection priority and skips comments; the full stack at an `any` halt; halts tried at place 0; TypeScript's `expression` as the one entry; html-derivative a language of its own; a covered second hand-off dropped; mixed repository rules compiled into the engine's table; ERB moves onto `Halts` in its second PR.
- [x] 1.2 Engine profile agreed with engine-spans (cost of a `&dyn` halt per place) → verify: their reply recorded in design section 9
  - engine-spans, 2026-11-03: `&dyn Fn` is fine; keep the `halt: None` branch; `#[inline]` dispatch; halts stay pure; label the PR `bench`.
- [x] 1.3 GraphQL port's embedding agreed with primitives → verify: their reply recorded in design section 10
  - primitives, 2026-11-03: `lex_embedding_until` maps onto `Halts`; nested hand-offs forward only the outer `any`; halts per rule from the grammar.
- [x] 1.4 nishiki-archive#159's dispatch agreed with go-lexer → verify: their reply recorded in design section 10
  - go-lexer, 2026-11-03: halts on the `Lexer`; `any` only in the halting instance; SQL, C, shell and GraphQL split too; Ruby's heredocs kept; `#[inline]` dispatch; the shared budget waits on core-fix.
- [ ] 1.5 Operator approves the design → verify: approval cited here

## 2. One halt contract

- [ ] 2.1 `Halts` and `Language::lex_until(…, &Halts)` on top of nishiki-archive#159 and the GraphQL port, with the rule engine and TypeScript honouring `before`, `after` and `any` → verify: HTML, GraphQL and Ruby heredoc fixtures unchanged, and a test per test kind checks the halt's place
- [ ] 2.2 The comment skip and the full state at an `any` halt → verify: a test halts inside a JavaScript string and resumes from the returned state
- [ ] 2.3 Halts tried at place 0 on a blank line → verify: a test with `^(\s*)(?=…)` as `any`
- [ ] 2.4 `Grammar::rule_halts`, replacing nishiki-archive#159's single `end` and `after` pair → verify: GraphQL and Ruby heredoc fixtures unchanged
- [ ] 2.5 The same split for SQL's and C's scans and for shell's and GraphQL's engines → verify: Callgrind at one codegen unit keeps each under 1.005 of its base
- [ ] 2.6 Ruby's `lex_until` on `&Halts` keeps its heredocs, through `lex_embedding_until` → verify: a Ruby region with `<<~SQL` embedded by ERB matches Shiki
- [ ] 2.7 Nested hand-offs forward the outer `any` → verify: ERB in a JavaScript string inside HTML halts JavaScript

## 3. Entries

- [ ] 3.1 `Entry` and `start_embedded(arena, base, entry)` → verify: every caller passes `Entry::Root` and no fixture changes

## 4. The engine hands off

- [ ] 4.0 core-fix's shared line budget, `Context { steps, open }`, has landed → verify: its PR cited here
- [ ] 4.1 `RuleKind::HandOff` and the generator's list of hand-lexed grammars and entries → verify: regenerating every engine table changes only tables that include a hand-lexed grammar
- [ ] 4.2 The hand-off frame and the resume at a halt → verify: Julia's oracle fixtures and a new input with `py"""` and `sql"""` match Shiki
- [ ] 4.3 Limits through the hand-off → verify: hostile lines inside an embedded region end at the work limit

## 5. Follow-ups

- [ ] 5.1 Julia's table without engine copies → verify: its compiled size before and after
- [ ] 5.2 ERB's any-depth injection (xml-html)
- [ ] 5.3 Perl, Elixir, Objective-C, PHP, Vue and Svelte, each its own PR
