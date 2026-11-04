# Tasks

Order of work (lead, 2026-11-04):

1. The engine as an embedded language, section 6 below, on the halt contract main has today (`end` and `after`).
2. The engine's hand-off to hand lexers, sections 4.1 to 4.3, once core-fix's shared line budget (nishiki-archive#228) lands.
3. HAML on the hand-off: Ruby, JavaScript, CSS and Markdown go to their hand lexers; Sass and CoffeeScript are engine rows of their own in the `textmate` tier, each after its licence is read at its source.
4. Ruby's `<<~HAML` heredoc embeds HAML.

Sections 1.5, 2 and 3 are deferred: `Halts`, `Grammar::rule_halts` and entries replace the two halt arguments in a cross-language PR of their own, and the engine's logic does not change when they do.

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

- [x] 4.0 core-fix's shared line budget, `Context { steps }`, with depth per language (design section 9), has landed → verify: its PR cited here
  - nishiki-archive#228, merged on 2026-11-04 as 2102df71: `Steps::lend` and `Finished`, with depth per language.
- [ ] 4.1 `RuleKind::HandOff` and the generator's list of hand-lexed grammars and entries → verify: regenerating every engine table changes only tables that include a hand-lexed grammar
- [ ] 4.2 The hand-off frame and the resume at a halt (design section 14) → verify: Julia's oracle fixtures and a new input with `py"""` and `sql"""` match Shiki, and an allocation test on an engine fence nested in a hand-lexed region allocates nothing per line
- [ ] 4.3 Limits through the hand-off → verify: hostile lines inside an embedded region end at the work limit

## 5. Follow-ups

- [ ] 5.1 Julia's table without engine copies → verify: its compiled size before and after
- [ ] 5.2 ERB's any-depth injection (xml-html)
- [ ] 5.3 Perl, Elixir, Objective-C, PHP, Vue and Svelte, each its own PR

## 6. The engine as an embedded language

- [x] 6.1 `textmate::lex_until` and `Language::lex_until` for every engine row: the line halts where `end` holds with only the root rule open, before the root's patterns, or where `after` holds and none of them match, plain lexing keeps no halt test (a type parameter compiles it out), and an embedded engine line, through `lex_until` or `lex_embedded`, starts from the work its limits lend and returns what it spent in `Finished` → verify: the halt and budget tests in `src/textmate/tests.rs`, and Callgrind at one codegen unit with every engine language at or under 1.000
  - [#325][pr325], merged on 2026-11-04 as 45d62854: the halt and budget tests pass in [CI][ci325], and [Callgrind][cg325] at one codegen unit reads every engine language at or under 1.000, with Nix at 0.9912.
    ERB, which does not run the engine, read 1.0011 on that run.
    It was judged noise by reruns of the same binaries on one machine: base 14,695,552 to 14,701,102 instructions, head 14,694,314 to 14,707,830, with no nishiki function's self cost or call count changed beyond the arena's hash maps.
- [ ] 6.2 Ruby's `<<~HAML` heredoc embeds HAML through it → verify: a Ruby fixture with a HAML heredoc matches Shiki

[pr325]: https://github.com/victor-software-house/nishiki-archive/pull/325
[ci325]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41800032873
[cg325]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41800032455
