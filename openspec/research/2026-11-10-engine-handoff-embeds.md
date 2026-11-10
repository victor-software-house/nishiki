# How the engine-handoff languages embed other languages now

Do Julia, Perl, Elixir, Objective-C, PHP, Vue and Svelte still compile copies of the grammars they embed?

Tasks 2.1 and 2.2 of the [`engine-handoff` change][tasks] assumed these languages would ship as TextMate engine tables.
Julia's table then held engine copies of the Python, JavaScript and SQL grammars ([proposal][proposal], Motivation item 2).
The tree read here is fad327b8.

## Answer

None of them compiles a copy.
Each has a lexer of its own on the rule engine, and none ships an engine table.
Each embedded region goes to that language's hand lexer.
The rule engine does it through `CarriedState::Embed`, and Vue through the HTML lexer.
Objective-C embeds nothing, because its grammar includes no other grammar.

## 1. Each language

| Language | Lexer of its own | Embeds through the hand lexers |
|:--|:--|:--|
| Julia | [#490][pr490], 2026-11-04 | Python, JavaScript and SQL |
| Perl | [#539][pr539], 2026-11-05 | HTML, CSS, JavaScript, SQL and XML |
| Elixir | [#528][pr528], 2026-11-05 | HTML |
| PHP | [#521][pr521], 2026-11-05 | HTML, CSS, JavaScript, JSON, SQL and XML |
| Svelte | [#517][pr517], 2026-11-05 | TypeScript, JavaScript, CoffeeScript, CSS, Less, Sass, SCSS, PostCSS and Markdown |
| Vue | [#478][pr478], 2026-11-04 | CSS and JavaScript, through the HTML lexer it runs on |
| Objective-C | [#514][pr514], 2026-11-05 | nothing |

- The embedded languages are the `embedded!` targets in each `src/<language>/grammar.rs` at fad327b8, found with `rg`.
  Julia's are its four `CarriedState::Embed` arms ([`src/julia/grammar.rs`][julia-grammar], around line 832).
  Julia's fourth arm embeds R, which [#490][pr490] left plain and a later change added.
- PHP's arms were read in full ([`src/php/grammar.rs`][php-grammar], around line 4511).
- The PR and date in each row are the commit that first gave the language a lexer on the rule engine, from `git log`.

## 2. Shiki's includes

Each grammar's includes of other grammars, read from Shiki 4.6.0's bundled grammars on 2026-11-10:

- Julia: `source.python`, `source.js`, `source.sql`, `source.r` and `source.cpp#root_context`.
  The C++ rule does not exist, so Shiki adds no C++ there ([#490][pr490]).
- Perl: `text.html.basic`, `text.xml`, `source.css`, `source.js`, `source.sql` and `source.postscript`.
- Elixir: `text.elixir` and `text.html.basic`.
- PHP: `text.html.basic`, `text.xml`, `source.sql`, `source.js`, `source.json` and `source.css`.
- Objective-C: none.
- Vue and Svelte: many, among them `source.ts`, `source.js`, `source.css` and the other style languages.

## 3. Julia's size

- With the engine copies, Julia added 707,720 bytes to a release build.
  This is from `feat/textmate-julia`, measured 2026-11-03 ([proposal][proposal]).
- With its own lexer, `size -A` puts Julia's own cost at about 83 KB ([#490][pr490], its size table against main 50b96ea4).
- The two figures come from different builds and bases.
  They show the order of the change, not an exact difference.

## 4. Grammars a caller brings

The TextMate engine now runs only grammars a caller brings.
Its compiler turns an include of a whole hand-lexed grammar into a hand-off ([`src/textmate/compile/tests.rs`][compile-tests], `mod hand_offs`).
The test `a_hand_lexed_grammar_missing_from_the_set_is_handed_off` covers it.

## Not verified

- Perl's, Elixir's and Svelte's embed lists come from `rg` over the generated code and the generators' headers, not from a run.
- No build measured Julia before and after on one base.
- Whether every Vue embed goes through the HTML lexer was not checked one by one.

## Compute

This Mac only: reading code, `rg`, `git log` and one Bun script over Shiki's grammars, a few minutes.

[tasks]: https://github.com/victor-software-house/nishiki-archive/blob/fad327b81aed702ea40e905dd35aa37903267403/openspec/changes/engine-handoff/tasks.md
[proposal]: https://github.com/victor-software-house/nishiki-archive/blob/fad327b81aed702ea40e905dd35aa37903267403/openspec/changes/engine-handoff/proposal.md
[julia-grammar]: https://github.com/victor-software-house/nishiki-archive/blob/fad327b81aed702ea40e905dd35aa37903267403/src/julia/grammar.rs
[php-grammar]: https://github.com/victor-software-house/nishiki-archive/blob/fad327b81aed702ea40e905dd35aa37903267403/src/php/grammar.rs
[compile-tests]: https://github.com/victor-software-house/nishiki-archive/blob/fad327b81aed702ea40e905dd35aa37903267403/src/textmate/compile/tests.rs
[pr478]: https://github.com/victor-software-house/nishiki-archive/pull/478
[pr490]: https://github.com/victor-software-house/nishiki-archive/pull/490
[pr514]: https://github.com/victor-software-house/nishiki-archive/pull/514
[pr517]: https://github.com/victor-software-house/nishiki-archive/pull/517
[pr521]: https://github.com/victor-software-house/nishiki-archive/pull/521
[pr528]: https://github.com/victor-software-house/nishiki-archive/pull/528
[pr539]: https://github.com/victor-software-house/nishiki-archive/pull/539
