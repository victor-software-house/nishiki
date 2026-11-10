# Tasks

Order of work (lead, 2026-11-04):

1. The engine as an embedded language, section 6 below, on the halt contract main has today (`end` and `after`).
2. The engine's hand-off to hand lexers, sections 4.1 to 4.3, once core-fix's shared line budget ([#228][pr228]) lands.
3. HAML on the hand-off:
   - Ruby, JavaScript, CSS and Markdown go to their hand lexers.
   - Sass and CoffeeScript are engine rows of their own in the `textmate` tier.
     Each follows once its licence is read at its source.
4. Ruby's `<<~HAML` heredoc embeds HAML.

Items 2 to 4 landed on 2026-11-04 (UTC):

- the tables in [#365][pr365];
- the tokenizer in [#366][pr366] and [#368][pr368];
- HAML with its heredoc in [#376][pr376];
- the limits test in [#377][pr377];
- Sass in [#381][pr381];
- CoffeeScript in [#388][pr388].

Sections 1.5, 2 and 3 are deferred.
`Halts`, `Grammar::rule_halts` and entries replace the two halt arguments in a cross-language PR of their own.
The engine's logic does not change when they do.

## 1. Agreement

- [x] 1.1 Halt semantics agreed with xml-html (ERB's halt at any depth, Vue's entries) → verify: their reply recorded in design section 9
  - xml-html, 2026-11-03:
    - `any` has injection priority and skips comments;
    - the full stack at an `any` halt, and halts tried at place 0;
    - TypeScript's `expression` as the one entry;
    - html-derivative a language of its own;
    - a covered second hand-off dropped;
    - mixed repository rules compiled into the engine's table;
    - ERB moves onto `Halts` in its second PR.
- [x] 1.2 Engine profile agreed with engine-spans (cost of a `&dyn` halt per place) → verify: their reply recorded in design section 9
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

## 4. The engine hands off

- [x] 4.0 core-fix's shared line budget, `Context { steps }`, with depth per language (design section 9), has landed → verify: its PR cited here
  - [#228][pr228], merged on 2026-11-04 as 2102df71: `Steps::lend` and `Finished`, with depth per language.
- [x] 4.1 The generator's list of hand-lexed grammars, a spec's `>` languages, and each list's hand-off (design section 14.3).
  → verify: regenerating every other engine table adds only `hand_offs: &[]`.
  HAML's test table splits each list at its hand-off (`a_hand_off_splits_its_list_in_two`).
  - [#365][pr365], merged on 2026-11-04 as f4ac6d41.
    Regenerating the 11 shipped tables and the 9 other test tables added one line to each, `hand_offs: &[]`.
    `a_hand_off_splits_its_list_in_two` runs on every table and passed in [CI][ci365].
- [x] 4.2 The hand-off frame and the resume at a halt (design section 14.4).
  → verify:
  - On HAML's test table, a filter goes to its language a line at a time.
  - JavaScript in an HTML test table's `<script>` halts at `</script>` inside a line.
  - The state carries across lines.
  - A PowerShell fence inside a `:markdown` filter allocates nothing after the first line.
  - `size -A` at one codegen unit measures the textmate tier's `.text` change.
  - Callgrind keeps every engine language at or under 1.000.
  - Julia's `py"""` and `sql"""` follow in 5.1.

  Evidence:
  - [#366][pr366], merged on 2026-11-04 as 49ff1aac, and [#368][pr368], merged as 410335fe.
  - The tests passed in [CI][ci366]:
    - a filter goes to its language a line at a time: `a_ruby_filter_hands_its_lines_to_ruby_and_takes_the_line_back_after`;
    - the `<script>` halt inside a line: `a_script_hands_javascript_its_text_up_to_the_end_tag`;
    - the state across lines: `a_style_hands_css_its_lines_across_the_document`;
    - no allocation after the first line: `lexing_a_nested_engine_fence_again_allocates_nothing`.
  - `size -A` at one codegen unit: `.text` grew 1,120 bytes in [#366][pr366].
    The `Handing` loop entered the binary with HAML, and [#376][pr376] measured it.
  - Callgrind at one codegen unit, rule 20 on a box:
    - [#366][pr366] read Haskell 1.0010, Julia 1.0008 and Mermaid 1.0018, the excess in libc `memcmp` on the arena maps;
    - [#368][pr368] then read them 0.9986, 0.9922 and 0.9948;
    - every engine language's product of the two rows is under 1.000.
    No single run measured [#368][pr368]'s head against [#366][pr366]'s base.
    CI's Callgrind gate passed on [#366][cg366] and [#368][cg368].
- [x] 4.3 Limits through the hand-off → verify: hostile lines inside an embedded region end at the work limit
  - [#377][pr377], merged on 2026-11-04 as fddcbdd0.
    `a_hand_off_spends_only_what_the_host_line_has_left` lexes three HAML documents as an embedded language.
    Their last lines hand off to Ruby, to JavaScript, and to PowerShell inside a Markdown fence.
    Each last line gets two steps of work, records the work limit and spends at most two steps.
    The test passed in [CI][ci377].

## 5. Follow-ups

- [ ] 5.1 Julia's table without engine copies → verify: its compiled size before and after
- [ ] 5.2 ERB's any-depth injection (xml-html)
- [ ] 5.3 Perl, Elixir, Objective-C, PHP, Vue and Svelte, each its own PR
- [ ] 5.4 A capture whose rule includes a hand-lexed grammar hands its text off, as CoffeeScript's backticks do → verify: the `coffee/coffeescript-javascript-literals` fixture matches Shiki
  - [#417][pr417] opened on 2026-11-04.
- [x] 5.5 Hand off a region whose end pattern uses `\G`, as Shiki does.
  → verify: the grammar links with the language named through `Grammars::hand_off`, and its fixture matches Shiki
  - Moved from the `grammar-api` change when it was archived.
  - `\G` matches only where the embedding rule's begin ended, and hand-lexed tokens never move that place.
    So the linker and `oracle/engine-tables.ts` no longer refuse `\G` in a halt: an end, a `before` rule or an `after` rule.
  - `an_end_at_the_anchor_matches_only_there` in `src/grammar/tests.rs` covers an end.
    `vue/vitepress-nav-menu-group` covers a `before` rule.
- [x] 5.6 Close the hand-off gaps that `src/textmate/tests/loaded.rs` lists.
  → verify: each fixture leaves its test's gaps, and the test passes
  - Moved from the `grammar-api` change when it was archived.
  - [x] 5.6.1 vue-directives' injection misses the tags that html-derivative's lexer reads: six Vue fixtures.
    - The cause was another: Vue's directive rules include `source.ts#expression`, a rule inside TypeScript's grammar.
    - Without TypeScript's grammar in the set, the linker drops those rules, as vscode-textmate does.
    - The test now loads the TypeScript, CSS and JSON grammars that Vue includes rules from.
    - `vue/vitepress-nav-menu-group` had a second cause.
      Vue's nested `<template>` rule lists `tag-stuff`, whose begin is `\G`, before html-derivative.
      The linker refused a hand-off whose halts use `\G`, so html-derivative's include matched nothing there.
      HTML's rule for invalid tags then took `<component>`.
      Task 5.5 lifts that refusal.
  - [x] 5.6.2 The hand-off goes on past a lexer that pops its host, as Less does at a `)`: `markdown/less-pop` and `vue/less-pop`.
    - The hand-off now ends the region where its language pops it, as vscode-textmate does.
    - Both fixtures left the gap lists, and the loaded tests pass.
  - [x] 5.6.3 Attribute names in an HTML fence differ from Shiki's: `markdown/invalid-tags` and both `markdown/perl-heredoc-tags-*`.
    - HTML told Markdown's document from its scope's number, which a loaded grammar's scope never has.
    - It now compares the scope's name.
  - [x] 5.6.4 `markdown/vitepress-runtime-api` holds a `vue` fence from markdown-vue's injection, which a Markdown-only set lacks.
    Its block should be left out, as other fences of missing languages are.
    - A blank line in the fence's body ended the block early, so the rest of the body was compared.
    - A block now runs over blank lines and ends at its closing fence.
- [x] 5.7 Give the JavaScript and TypeScript that a loaded Svelte grammar hands off Svelte's `$` prefixes.
  → verify: `javascript_in_a_loaded_svelte_grammar_takes_the_dollar_prefixes` passes without `lang-svelte`
  - Svelte's grammar injects the prefixes into `source.ts` and `source.js`.
    A handed region's rules never run, so the injection never applied there.
  - The engine now lexes such a region with the TypeScript lexer's Svelte dialect, when the root grammar is `source.svelte`.
    vscode-textmate applies the injections of the grammar it lexes with, so only the root's count.
  - The dialect and HTML's Svelte root are now built with the engine too, not only with `lang-svelte`.

## 6. The engine as an embedded language

- [x] 6.1 `textmate::lex_until` and `Language::lex_until` for every engine row.
  - The line halts where `end` holds with only the root rule open, before the root's patterns.
    It also halts where `after` holds and none of them match.
  - Plain lexing keeps no halt test: a type parameter compiles it out.
  - An embedded engine line, through `lex_until` or `lex_embedded`, starts from the work its limits lend.
    It returns what it spent in `Finished`.

  → verify: the halt and budget tests in `src/textmate/tests.rs`, and Callgrind at one codegen unit with every engine language at or under 1.000.
  - [#325][pr325], merged on 2026-11-04 as 45d62854.
    The halt and budget tests pass in [CI][ci325].
  - [Callgrind][cg325] at one codegen unit reads every engine language at or under 1.000.
    Nix reads 0.9912.
  - ERB, which does not run the engine, read 1.0011 on that run.
    Reruns of the same binaries on one machine showed it as noise:
    - base: 14,695,552 to 14,701,102 instructions;
    - head: 14,694,314 to 14,707,830.

    No nishiki function's self cost or call count changed beyond the arena's hash maps.
- [x] 6.2 Ruby's `<<~HAML` heredoc embeds HAML through it → verify: a Ruby fixture with a HAML heredoc matches Shiki
  - [#376][pr376], merged on 2026-11-04 as 3cdfc138.
    In `ruby/coverage.txt`, three lines of the `<<~HAML` heredoc take HAML's scopes, and the oracle check passed in [CI][ci376].

[pr159]: https://github.com/victor-software-house/nishiki-archive/pull/159
[pr228]: https://github.com/victor-software-house/nishiki-archive/pull/228
[pr325]: https://github.com/victor-software-house/nishiki-archive/pull/325
[pr365]: https://github.com/victor-software-house/nishiki-archive/pull/365
[pr366]: https://github.com/victor-software-house/nishiki-archive/pull/366
[pr368]: https://github.com/victor-software-house/nishiki-archive/pull/368
[pr376]: https://github.com/victor-software-house/nishiki-archive/pull/376
[pr377]: https://github.com/victor-software-house/nishiki-archive/pull/377
[pr381]: https://github.com/victor-software-house/nishiki-archive/pull/381
[pr388]: https://github.com/victor-software-house/nishiki-archive/pull/388
[pr417]: https://github.com/victor-software-house/nishiki-archive/pull/417
[ci365]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41837615709
[ci366]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41837614919
[ci376]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41837829593
[ci377]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41841244689
[cg366]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41837614186
[cg368]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41837615905
[ci325]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41800032873
[cg325]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41800032455
