# Tasks

Order of work (lead, 2026-11-04):

1. The engine as an embedded language, section 3 below, on the halt contract main has today (`end` and `after`).
2. The engine's hand-off to hand lexers, tasks 1.1 to 1.3, once core-fix's shared line budget ([#228][pr228]) lands.
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

`Halts`, `Grammar::rule_halts` and entries are deferred to the [`halt-contract` change][contract].

## 1. The engine hands off

- [x] 1.0 core-fix's shared line budget, `Context { steps }`, with depth per language (the [`halt-contract` design][contract], section 9), has landed → verify: its PR cited here
  - 2026-11-04: [#228][pr228] merged as 2102df71, adding `Steps::lend` and `Finished`, with depth per language.
- [x] 1.1 The generator's list of hand-lexed grammars, a spec's `>` languages, and each list's hand-off (design section 3).
  → verify: regenerating every other engine table adds only `hand_offs: &[]`.
  HAML's test table splits each list at its hand-off (`a_hand_off_splits_its_list_in_two`).
  - 2026-11-04: [#365][pr365] merged as f4ac6d41.
    Regenerating the 11 shipped tables and the 9 other test tables added one line to each, `hand_offs: &[]`.
    `a_hand_off_splits_its_list_in_two` runs on every table and passed in [CI][ci365].
- [x] 1.2 The hand-off frame and the resume at a halt (design section 4).
  → verify:
  - On HAML's test table, a filter goes to its language a line at a time.
  - JavaScript in an HTML test table's `<script>` halts at `</script>` inside a line.
  - The state carries across lines.
  - A PowerShell fence inside a `:markdown` filter allocates nothing after the first line.
  - `size -A` at one codegen unit measures the textmate tier's `.text` change.
  - Callgrind keeps every engine language at or under 1.000.
  - Julia's `py"""` and `sql"""` follow in 2.1.

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
- [x] 1.3 Limits through the hand-off → verify: hostile lines inside an embedded region end at the work limit
  - [#377][pr377], merged on 2026-11-04 as fddcbdd0.
    `a_hand_off_spends_only_what_the_host_line_has_left` lexes three HAML documents as an embedded language.
    Their last lines hand off to Ruby, to JavaScript, and to PowerShell inside a Markdown fence.
    Each last line gets two steps of work, records the work limit and spends at most two steps.
    The test passed in [CI][ci377].

## 2. Follow-ups

- [ ] 2.1 Julia's table without engine copies → verify: its compiled size before and after
- [ ] 2.2 Perl, Elixir, Objective-C, PHP, Vue and Svelte, each its own PR
- [x] 2.3 A capture whose rule includes a hand-lexed grammar hands its text off, as CoffeeScript's backticks do → verify: the `coffee/coffeescript-javascript-literals` fixture matches Shiki
  - 2026-11-04: [#417][pr417] opened, and merged the same day.
  - 2026-11-10: `oracle::every_language_matches_shiki` compares the `coffee/coffeescript-javascript-literals` fixture.
    It passed on main at fab8c57c ([run][run-main-oracle]).
- [x] 2.4 Hand off a region whose end pattern uses `\G`, as Shiki does.
  → verify: the grammar links with the language named through `Grammars::hand_off`, and its fixture matches Shiki
  - Moved from the `grammar-api` change when it was archived.
  - `\G` matches only where the embedding rule's begin ended, and hand-lexed tokens never move that place.
    So the linker and `oracle/engine-tables.ts` no longer refuse `\G` in a halt: an end, a `before` rule or an `after` rule.
  - `an_end_at_the_anchor_matches_only_there` in `src/grammar/tests.rs` covers an end.
    `vue/vitepress-nav-menu-group` covers a `before` rule.
- [x] 2.5 Close the hand-off gaps that `src/textmate/tests/loaded.rs` lists.
  → verify: each fixture leaves its test's gaps, and the test passes
  - Moved from the `grammar-api` change when it was archived.
  - [x] 2.5.1 vue-directives' injection misses the tags that html-derivative's lexer reads: six Vue fixtures.
    - The cause was another: Vue's directive rules include `source.ts#expression`, a rule inside TypeScript's grammar.
    - Without TypeScript's grammar in the set, the linker drops those rules, as vscode-textmate does.
    - The test now loads the TypeScript, CSS and JSON grammars that Vue includes rules from.
    - `vue/vitepress-nav-menu-group` had a second cause.
      Vue's nested `<template>` rule lists `tag-stuff`, whose begin is `\G`, before html-derivative.
      The linker refused a hand-off whose halts use `\G`, so html-derivative's include matched nothing there.
      HTML's rule for invalid tags then took `<component>`.
      Task 2.4 lifts that refusal.
  - [x] 2.5.2 The hand-off goes on past a lexer that pops its host, as Less does at a `)`: `markdown/less-pop` and `vue/less-pop`.
    - The hand-off now ends the region where its language pops it, as vscode-textmate does.
    - Both fixtures left the gap lists, and the loaded tests pass.
  - [x] 2.5.3 Attribute names in an HTML fence differ from Shiki's: `markdown/invalid-tags` and both `markdown/perl-heredoc-tags-*`.
    - HTML found Markdown's document by its scope's number, which a loaded grammar's scope never has.
    - It now compares the scope's name.
  - [x] 2.5.4 `markdown/vitepress-runtime-api` holds a `vue` fence from markdown-vue's injection, which a Markdown-only set lacks.
    Its block should be left out, as other fences of missing languages are.
    - A blank line in the fence's body ended the block early, so the rest of the body was compared.
    - A block now runs over blank lines and ends at its closing fence.
- [x] 2.6 Give the JavaScript, TypeScript and CoffeeScript that a loaded Svelte grammar hands off Svelte's `$` prefixes.
  → verify: `javascript_in_a_loaded_svelte_grammar_takes_the_dollar_prefixes` and
  `coffeescript_in_a_loaded_svelte_grammar_takes_the_dollar_prefixes` pass without `lang-svelte`
  - Svelte's grammar injects the prefixes into `source.ts`, `source.js` and `source.coffee`.
    A handed region's rules never run, so the injection never applied there.
  - The engine now lexes such a region with the TypeScript lexer's Svelte dialect, when the root grammar is `source.svelte`.
    vscode-textmate applies the injections of the grammar it lexes with, so only the root's count.
  - The dialect and HTML's Svelte root are now built with the engine too, not only with `lang-svelte`.
  - CoffeeScript takes its own lexer's Svelte dialect the same way.
- [x] 2.7 Take `\G` at a line's start from the rule that embeds a hand-off's language.
  → verify: `make_reads_the_anchor_of_a_begin_that_took_the_line_feed` passes, with the scopes Shiki gives
  - The hand-off frame kept no begin-at-EOL flag, so `\G` never matched at a later line's start.
  - vscode-textmate reads the flag from the innermost open rule.
    While the language has only its root open, that is the embedding rule.
    With a context of the language open, that context's begin decides.
  - Of the hand-lexed grammars, only Markdown, Make and C++ have `\G` among their root patterns.
    At a line's start, only Make's `(^ *|\G\s*)` matches where `^` does not.
  - The test checks Make at its root, Make inside `ifdef`, and a begin that stops before the line feed.

## 3. The engine as an embedded language

- [x] 3.1 `textmate::lex_until` and `Language::lex_until` for every engine row.
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
- [x] 3.2 Ruby's `<<~HAML` heredoc embeds HAML through it → verify: a Ruby fixture with a HAML heredoc matches Shiki
  - 2026-11-04: [#376][pr376] merged as 3cdfc138.
    In `ruby/coverage.txt`, three lines of the `<<~HAML` heredoc take HAML's scopes, and the oracle check passed in [CI][ci376].

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
[run-main-oracle]: https://github.com/victor-software-house/nishiki-archive/actions/runs/42855984002
[ci365]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41837615709
[ci366]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41837614919
[ci376]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41837829593
[ci377]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41841244689
[cg366]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41837614186
[cg368]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41837615905
[ci325]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41800032873
[cg325]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41800032455
[contract]: ../halt-contract/design.md
