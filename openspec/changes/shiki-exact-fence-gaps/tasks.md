# Tasks

Each fix is measured as design section 1 says: on a box, at seeds 1, 2 and 3.
Design section 2 names the runs that found each case.

## 1. Markdown

- [ ] 1.1 A `vue` fence opens in a paragraph's line after four spaces, as in Shiki.
  → verify: a test holds a paragraph, a blank line, and `x` and a `vue` fence each after four spaces, with Shiki's scopes
  - 2026-11-12: a Shiki probe in `shiki-exact-gaps` task 6.4's work shows it.
    The paragraph runs on past four spaces, and the injection opens the fence inside it.
    nishiki opens no fence there.
- [ ] 1.2 A `vue` fence in a Vue body inside a Markdown fence in a quote sits on Shiki's scopes.
  → verify: a test holds the case with Shiki's scopes, and no Markdown `mdw` document differs at any seed
  - 2026-11-12: Markdown's `mdw` corpus at seed 3 shows it in document 12, by 64 bytes, at `> ```vue:no-line-numbers`.
    Shiki puts `meta.tag-stuff` under the fence, and nishiki puts `source.ts`.

## 2. Vue fences in other languages

- [ ] 2.1 In a C# fence, the line after a `//` or `///` comment opens a `vue` fence on Shiki's scopes.
  → verify: a test holds both comments before a `vue` fence with Shiki's scopes, and the sweep's C# documents match
  - 2026-11-12: the sweep shows it in six of C#'s eight documents.
    Shiki's comment `while` rule fails on the fence's line, and the fence sits on the C# body.
    nishiki keeps the comment's scope under the fence.
- [ ] 2.2 A `vue` fence in a PHP heredoc that holds XML sits on Shiki's scopes.
  → verify: a test holds the case with Shiki's scopes, and the sweep's PHP documents match
  - 2026-11-12: the sweep shows it in PHP's document 1.
    Shiki puts `meta.tag.xml` under the fence, and nishiki leaves it out.
- [ ] 2.3 A Ruby heredoc's closing line ends where Shiki ends it, after a `vue` fence opens in its body.
  → verify: a Shiki probe names the rule that ends there, a test holds the case, and the sweep's Ruby documents match
  - 2026-11-12: the sweep shows it in Ruby's document 6.
    Both open the fence inside a `<<~RUBY` heredoc.
    At the `RUBY` line, Shiki ends the heredoc, and nishiki keeps the fence open.
- [ ] 2.4 In a `vue` fence, an attribute name that holds a `.` reads as a Vue directive, as in Shiki.
  → verify: a test holds `.size = size,` and `TEncoding.UTF8.GetString(` in an open tag with Shiki's scopes, and the sweep's Pascal and R documents match
  - 2026-11-12: the sweep shows it in Pascal's document 2 and R's document 4.
    Shiki gives `meta.attribute.directive.vue` and TypeScript.
    nishiki gives an unrecognized HTML attribute.

## 3. Svelte

- [ ] 3.1 A Svelte block in a TypeScript template string, in an `{@const}` type annotation, opens where Shiki opens it.
  → verify: a test holds `{#if` in the string with Shiki's scopes, and no Svelte bare differential document differs at any seed
  - 2026-11-12: Svelte's bare differential at seed 2 shows it in document 46, by 302 bytes, at line 20.
    Shiki opens `meta.special.if.svelte` inside `string.template.ts`, and nishiki keeps the string.

## 4. LaTeX

- [ ] 4.1 Lua in a LaTeX `\verb` argument in a quote gives its words Shiki's scopes.
  → verify: a test holds the case with Shiki's scopes, and no LaTeX `mdw` document differs at any seed
  - 2026-11-12: seed 2's `mdw` corpus of LaTeX shows it in document 87, by 13 bytes.
    Shiki gives `title` `variable.other.lua`, and nishiki gives it `source.lua` alone.

## 5. Bat, Python and YAML

- [ ] 5.1 Bat gives a `rem` comment in a quote's fence Shiki's scopes.
  → verify: a test holds the case with Shiki's scopes, and no Bat `mdw` document differs at any seed
  - 2026-11-12: seed 2's `mdw` corpus of Bat shows it in document 40, by 70 bytes.
    The line holds a non-ASCII digit, `٣`.
    Shiki opens `comment.line.rem.batchfile`, and nishiki opens nothing.
- [ ] 5.2 Python leaves a raw f-string where Shiki leaves it, in a fence in a list.
  → verify: a test holds the case with Shiki's scopes, and no Python `mdw` document differs at any seed
  - 2026-11-12: seed 2's `mdw` corpus of Python shows it in document 2, by 127 bytes.
    At line 10, nishiki keeps a raw f-string open, and Shiki has closed it.
- [ ] 5.3 YAML gives a key in a quote's fence Shiki's scopes, such as `complex4:`.
  → verify: a test holds the case with Shiki's scopes, and no YAML `mdw` document in a quote differs at any seed
  - 2026-11-12: seed 2's `mdw` corpus of YAML shows it in document 27, by 1 byte.
- [ ] 5.4 YAML gives `-:: !circle` Shiki's scopes.
  → verify: a test holds the line with Shiki's scopes, and no YAML `mdw` document differs at any seed
  - 2026-11-12: seed 2's `mdw` corpus of YAML shows it in document 28, by 2 bytes.
    Shiki gives the key `entity.name.tag.yaml`, and nishiki leaves it out.

## 6. PHP, Blade and CSS

- [x] 6.1 Blade's PHP gives a typed parameter after U+00A0 Shiki's scopes.
  → verify: a test holds the case with Shiki's scopes, and no ASCII-only stand-in for `\s` is left in `src/php/` or `src/blade/`
  - 2026-11-12: the case is in `tests/oracle/blade/coverage.txt`, with each space inside a tag turned into U+00A0.
    On a box, at lines 376 and 378, nishiki gives no `meta.function.parameter.typehinted.php storage.type.php`, and Shiki does.
  - 2026-11-12: Blade's name class, `[_a-z\x7F-ÿ]`, holds U+0085 and U+00A0, which are `\s` too.
    In `(NAME)\s+((&)?\s*(\.\.\.)?(\$+)NAME)`, Oniguruma gives back the type's last character for `\s+`.
    nishiki read the name without giving any back.
  - 2026-11-12: PHP gave back every trailing space past ASCII.
    Shiki gives back one: `S` and two U+00A0 before `$j` make the type `S` and one U+00A0.
  - 2026-11-12: both lexers read `^\s*(LABEL)`, a heredoc's or a nowdoc's closing line, with ASCII white space only.
    Shiki closes the heredoc after U+00A0 or U+3000.
  - 2026-11-12: four tests hold the cases with Shiki's scopes, two in each lexer.
    They are `a_type_gives_back_its_last_unicode_space_for_the_variable` and `a_heredoc_label_after_unicode_white_space_closes_it`.
  - 2026-11-12: the other ASCII-only white space in both lexers is the class `[-\t !,.?]` of a comment's text, which holds no `\s`.
  - 2026-11-12: on a box, PHP's `win` corpora at seeds 1, 2 and 3, 300 documents each: none differs with the fix, nor at seed 1 on the base.
- [x] 6.2 In PHP and Blade, a `\s*` or `\s+` before a name gives the name its last space past ASCII, as Oniguruma does.
  → verify: a test holds `^\s*(NAME)\s*(?=:)` after U+3000 with Shiki's scopes, and no document differs in PHP's `win` corpora with U+00A0 or U+3000 for each space
  - 2026-11-12: those corpora, on a box, at seeds 1, 2 and 3: 13, 10 and 14 of 300 documents differ with task 6.1's fix, and seed 1's same 13 on the base.
    Each is a line of spaces, `:` and `$value`, where Shiki gives the last space `entity.name.goto-label.php`.
  - 2026-11-12: the matchers read `\s*` to its end and then need a name.
    Oniguruma gives the last space back when no name starts there, and U+00A0 and U+3000 are name characters.
    About 16 matchers in `src/php/` and `src/blade/` have that shape.
  - 2026-11-12: `Lexer::after_spaces` in `src/php_family/matchers.rs` reads the run and tries the part after it again at the run's last space past ASCII.
    Each such matcher in PHP and Blade calls it.
    Possessive runs give nothing back and keep their reads.
  - 2026-11-12: `white_space_gives_its_last_unicode_space_to_a_name`, in each lexer, holds Shiki's scopes for labels, calls, `class` and `goto`.
    Both fail on the base and pass with the fix, on a box.
  - 2026-11-12: on a box, the corpora at seeds 1 and 3 and the 20 documents from the fixtures: none differs.
    At seed 2, one document differs, by 4 bytes, in CSS inside a heredoc.
    The line is `CSS` and three U+00A0, where Shiki gives the last two `meta.selector.css`.
  - 2026-11-12: with task 6.3's fix, on a box, that corpus at seed 2: none differs.
- [x] 6.3 CSS opens a selector at a wide space that follows another, as Shiki does.
  → verify: a test holds `CSS` and three U+00A0 with Shiki's scopes, and no document differs in CSS's `win` corpora with U+00A0 or U+3000 for each space
  - 2026-11-12: task 6.2's PHP corpus at seed 2 shows it in a heredoc's CSS.
    Shiki gives the last two U+00A0 `meta.selector.css`, and nishiki gives the run `source.css`.
  - 2026-11-12: the selector begin of Shiki's CSS grammar is `(?<=^|(^|[^:])\s|[;{}]|…)(?=[…[^\x00-\x7F]]|…)`.
    At the second U+00A0, the lookbehind reads `[^:]\s` and the class `[^\x00-\x7F]` takes the space.
    The selector opens and closes there without advancing.
    It keeps the rest of the line.
  - 2026-11-12: `Lexer::pass_wide_space` in `src/rules/lexer.rs` passes a run of one repeated wide space in one step.
    It tried the first U+00A0 only.
  - 2026-11-12: `looks_behind_at_a_wide_space` in `src/unstable/codegen/blank_runs.rs` finds a pattern with a lookbehind that can pass after a wide space and a start that can be one.
    `codegen/src/blank_runs.rs` writes each grammar's `rules::WideSpaceLookbehind` impl from it.
    The scan of a grammar it marks tries each wide space of a run.
    It marks 25 of the 48 rule-engine grammars.
  - 2026-11-12: `a_selector_opens_at_a_wide_space_after_another` holds Shiki's scopes for U+00A0 and U+3000.
    It fails on the base and passes with the fix, on a box.
  - 2026-11-12: on a box, CSS's `win` corpora at seeds 1, 2 and 3, 300 documents each, with U+00A0 or U+3000 for each space: none differs.
    Nor does any of the plain `win` corpora at those seeds.
- [ ] 6.4 A pattern that matches only at the last wide space of a run matches there, as in Shiki.
  → verify: a test holds a run of U+00A0 before such a pattern with Shiki's scopes
  - 2026-11-12: such a pattern is `\s(?!\s)`.
    It fails at each wide space of a run but the last.
    The scan of a `BlankRuns::Skip` grammar tries a run of one repeated wide space at its first space only.
    `BlankRuns::ToLast` tries such patterns at the last blank of a run of spaces and tabs only.

## 7. Vue fences where a hand lexer's rule begins

Shiki opens a `vue` fence mid-line where a rule's begin match ends, since `\G` holds there.
Each hand lexer below needs the halt that task 6.6 of [gaps] gave TypeScript.

Each verify is the same: a test holds the openers named with Shiki's scopes, and the mid-line sweep shows no document of that language differing.
The sweep puts each of 20 openers before a `vue` fence in a fence of each language.

- [ ] 7.1 JSON, JSONC and JSON Lines open a `vue` fence where Shiki opens one.
  - 2026-11-12: the sweep shows it after `"`, `/*`, `[`, `{`, `r"` and `${`.
- [ ] 7.2 YAML opens a `vue` fence where Shiki opens one.
  - 2026-11-12: the sweep shows it after `"`, `'`, `(`, `{`, `#`, `--`, `%`, `;` and `x = <a>`.
- [ ] 7.3 INI opens a `vue` fence where Shiki opens one.
  - 2026-11-12: the sweep shows it after `"`, `'`, `#`, `;` and `r"`.
- [ ] 7.4 Rust opens a `vue` fence where Shiki opens one.
  - 2026-11-12: the sweep shows it after `"`, `'`, `/*` and `r"`.
- [ ] 7.5 Vue opens a `vue` fence where Shiki opens one.
  - 2026-11-12: the sweep shows it after `<a>`, `x = <a>`, `<!--` and `<<EOF`.
  - 2026-11-12: so do Vue's `mdw` documents 16 at seed 1, 25 at seed 2, and 17 and 76 at seed 3.
    In each, TypeScript or JavaScript in a `<script>` reaches a quote's `vue` fence line.
- [ ] 7.6 SCSS opens a `vue` fence where Shiki opens one.
  - 2026-11-12: the sweep shows it after `/*`, `{`, `//` and `${`.
- [ ] 7.7 XML, HTML and Markdown open a `vue` fence after `<!--`, as Shiki does.
  - 2026-11-12: the sweep shows it in each, and in XML after `<<EOF` too.
- [ ] 7.8 Diff opens a `vue` fence after `#`, as Shiki does.
  - 2026-11-12: the sweep shows it.

[gaps]: ../shiki-exact-gaps/tasks.md
