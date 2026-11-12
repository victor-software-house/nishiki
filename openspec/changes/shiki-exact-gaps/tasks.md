# Tasks

Each fix is measured as design section 1 says: on a box, at seeds 1, 2 and 3.
The [research note][note] holds the numbers each task starts from.

## 1. `$base`

- [x] 1.1 Objective-C and Objective-C++ try the root patterns of the document that embeds them at each `$base` include, as design section 2 says.
  → verify: at every seed, no document differs in Objective-C's fenced differential, `mdw` and `mdg` corpora.
  Nor does one differ in Objective-C++'s `win` corpus and bare differential, and the oracle test passes.
  - 2026-11-11: Objective-C's fenced differential, on a box, 100 documents per seed.
    Before, 89, 85 and 90 differed at seeds 1, 2 and 3.
    After, none differs at any seed.
  - 2026-11-11: Objective-C's `mdw` corpus, on a box, 100 documents per seed.
    Before, 87, 88 and 84 differed at seeds 1, 2 and 3.
    After, none differs at any seed.
  - 2026-11-11: Objective-C's `mdg` corpus, on a box, 100 documents per seed.
    Before, 87, 95 and 90 differed at seeds 1, 2 and 3.
    After, none differs at any seed.
  - 2026-11-11: Objective-C++'s `win` corpus and bare differential, on a box.
    No document differs at any seed, before or after.
    The oracle test passes.
  - 2026-11-11: Callgrind against main, three builds per side, reads Objective-C at 0.9996 for lexing.

## 2. The fuzz script

- [x] 2.1 `oracle/fuzz.ts` refuses `gen` and `mdg` alike for a language with no pieces.
  → verify: both modes give the same answer for C#, Lua, R and Swift, and the pull request records where their `gen` documents came from
  - 2026-11-12: the guard tested `mode.endsWith("g")`, which `gen` fails.
    Without pieces, `gen` drew from an empty list.
    Each line held the word `undefined`, repeated.
  - 2026-11-12: locally, `gen` and `mdg` both exit 2 for C#, Lua, R and Swift, with "needs pieces".
    C++, which has pieces, still writes both.

## 3. Rust

- [x] 3.1 Rust reads non-ASCII digits and names with the classes of Shiki's engine, after reading the grammar's exact classes.
  → verify: at every seed, no document differs in Rust's `win` and `mdw` corpora
  - 2026-11-12: the grammar's Unicode classes are `\w`, `\b` and `\d`.
    `\d` sits in its decimal, hex and escape digits.
    Every other class is an ASCII range.
  - 2026-11-12: Rust's `win` corpus, on a box, 300 documents per seed.
    Before, 3, 3 and 1 differed at seeds 1, 2 and 3.
    After, none differs at any seed.
  - 2026-11-12: Rust's `mdw` corpus, on a box, 100 documents per seed.
    Before, 5, 3 and 0 differed at seeds 1, 2 and 3.
    After, none differs at any seed.
  - 2026-11-12: Callgrind against the base, one build per side, reads Rust at 0.9893 for lexing.

## 4. Svelte and seed 2

- [x] 4.1 Svelte lexes `{#each items as item, $i}{/each}` within the default work limit, as Shiki does.
  → verify: a test holds the line with Shiki's scopes, and no Svelte `win` document stops or differs at any seed
  - 2026-11-12: at [98bb9820][base], Svelte's `win` corpus at seed 2 holds the line in documents 2, 17, 76 and 91.
    The corpus test stops at document 17, where the line reaches the work limit.
  - 2026-11-12: the line stops only after `{#each items as item, i{/each}`, which leaves the tag open.
    Any line that starts with `{` stops there.
    Alone, the line matches Shiki.
  - 2026-11-12: the cause is in the rule engine.
    The `{` opens the `{#each}` tag's destructuring, whose text goes to TypeScript.
    After its `}`, the destructuring opens again and TypeScript halts at once.
  - 2026-11-12: vscode-textmate then closes the rule without advancing and stops the line.
    nishiki cleared where rules opened at each hand-over, so the rule opened again until the work limit.
  - 2026-11-12: Svelte's `win` corpus, on a box, 100 documents per seed.
    Before, seed 2 stopped and seed 3 had one document that differs.
    After, no document stops, and seed 3's one document still differs.
    That document is task 4.2's case.
    Svelte's `mdw` corpus had no document that differs before or after.
  - 2026-11-12: Identity changes three edited copies of Svelte's `capture-ends` fixture.
    Each now matches Shiki.
  - 2026-11-12: Callgrind against the base, one build per side, reads Svelte at 1.0022 for lexing.
    No gated row reads over 1.0039.
- [x] 4.2 After `<script lang=">let one: number = 1;</script>`, Svelte enters the embedded script where Shiki does.
  So do the `<script lang="coffee">` lines after it.
  → verify: a test holds the case with Shiki's scopes, and no Svelte `win` document differs at any seed
  - 2026-11-12: at [98bb9820][base], Svelte's `win` corpus at seed 3 shows it.
    Shiki enters `meta.embedded.block.svelte source.js`.
    nishiki stays in `string.quoted.svelte`.
  - 2026-11-12: the bodies are `L:` injections.
    Every rule inside the tag's start tries them first, the attribute's string included.
    So the `>` in the string opens the body.
    nishiki tried them only in the tag's own two rules.
  - 2026-11-12: the rules a language tag's start opens now try one pattern that stands for every body.
    Nine separate patterns there cost Svelte 2.9% more instructions.
  - 2026-11-12: Svelte's `win` and `mdw` corpora, on a box, 100 documents per seed.
    No document differs at any seed.
    Before, seed 3's `win` corpus had one, this task's case.
  - 2026-11-12: Identity changes two edited copies of Svelte's fixtures, `coverage` and `kit-root`.
    Each now matches Shiki.
  - 2026-11-12: Callgrind against the base, one build per side, reads Svelte at 1.0024 for lexing.
    No gated row reads over that.
- [x] 4.3 Find the language of the document that differs at seed 2.
  → verify: the language and the document are recorded here, and its difference gets a task or a fix
  - 2026-11-12: at [98bb9820][base], the `win` corpora at seed 2 hold 4,796 documents without Svelte's four.
    One of them differs, by 151 bytes.
    Its language was not recorded.
  - 2026-11-12: on a box, at [d36a1da6][s2-head], seed 2 wrote 100 `win` documents for each of the 73 languages with oracle fixtures.
    Two documents differ, and the other 71 languages have none.
    At [98bb9820][base], the same two differ by the same bytes.
  - 2026-11-12: the 151 bytes are Ruby's document 76.
    Its line 2 ends in `%W`.
    nishiki keeps the literal open to line 9, where Shiki closes it at the end of line 3.
    That is task 5.1's case.
  - 2026-11-12: Vue's document 78 differs by 12 bytes, at `<styl/> c` and the `</st` after it.
    Task 6.3 holds it.

## 5. Ruby and Mermaid

- [ ] 5.1 A `%w` or `%W` at the end of a line opens a percent literal that ends with the next line, as in Shiki.
  → verify: at every seed, no document differs in Ruby's `win` corpus, and the research note's document 296 matches
  - 2026-11-12: Shiki 4.6.0 reads the line's newline as the literal's delimiter.
    In `a = %W`, then `b c`, then `d = 1`, the string holds `b c`.
    `d` is `variable.ruby`.
    `%w` gives the same, with `string.quoted.other.ruby`.
  - 2026-11-12: Ruby's `win` corpus at seed 2 shows it with `%W`, in document 76 (task 4.3).
  - 2026-11-12: the begin capture held the line feed, which lies outside the line's text.
    So the literal carried an empty delimiter, and its end never matched.
    An empty delimiter now closes the literal at the next line feed, as Elixir's sigils do.
  - 2026-11-12: two tests hold documents 296 and 76 with Shiki's scopes.
    On a box, both fail before the fix and pass after it.
  - 2026-11-12: Ruby's `win` corpus, on a box, 300 documents per seed.
    Before, seed 1 had two documents that differ, 296 and 59.
    Seed 2 had document 76 (task 4.3).
    After, seeds 2 and 3 have none, and documents 296 and 76 match.
  - 2026-11-12: seed 1's one document that differs is task 5.2's document 59, by 2 bytes.
    So this task's verify clause passes once task 5.2 lands.
  - 2026-11-12: Identity changes no fixture.
    Callgrind against the base, one build per side, reads Ruby at 1.0001 for lexing.
    No gated row reads over 1.0012.
- [ ] 5.2 After a here document that runs on past a blank line, the next line's command name gets Shiki's shell scopes.
  → verify: the research note's document 59 matches, and no Ruby `win` document differs at any seed
  - 2026-11-12: at [98bb9820][base], Ruby's `win` corpus at seed 1 shows it, at line 12, column 1, in `sh = <<~'SH'`.
    Shiki gives `meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell`.
    nishiki stops at `meta.statement.command.shell`.
- [ ] 5.3 In a fence, the text after a line's second here document opener gets the embed Shiki gives it.
  → verify: at every seed, no document differs in Ruby's `mdw` corpus
- [ ] 5.4 Mermaid leaves a backslash after a closed string plain, as Shiki does.
  → verify: at every seed, no document differs in Mermaid's `win` and `mdw` corpora and its differential
  - 2026-11-12: at [98bb9820][base], Mermaid's `win` corpus at seed 1 shows it, at line 38, column 32.
    The line holds `click B "https://example.com"\` after two blanks.
    Shiki gives `markdown.mermaid.codeblock`, and nishiki adds `string`.

## 6. Vue

- [ ] 6.1 Find why Vue's regions end at other lines in a list-item fence.
  → verify: the cause is recorded here, with the rule it comes from
- [ ] 6.2 Vue in a list-item fence ends its regions where Shiki does.
  → verify: at every seed, no document differs in Vue's `mdw` corpus
- [x] 6.3 Vue gives `<styl/>` and the `</st` after it Shiki's scopes.
  → verify: a test holds the case with Shiki's scopes, and no Vue `win` document differs at any seed
  - 2026-11-12: Vue's `win` corpus at seed 2 shows it in document 78, at [98bb9820][base] and at [d36a1da6][s2-head].
    At line 9, column 1, Shiki gives `self-closing-tag punctuation.definition.tag.begin.html.vue`.
    nishiki gives no `self-closing-tag`.
  - 2026-11-12: at line 10, column 1, Shiki gives `</` the scope `punctuation.definition.tag.begin.html.vue`.
    nishiki gives `meta.tag-stuff invalid.illegal.character-not-allowed-here.html`.
  - 2026-11-12: Shiki 4.6.0 scopes `<div/>` and `<style/>` as it scopes `<styl/>`.
  - 2026-11-12: the rule is `(<)([-0-:A-Za-z]+)(?=([^>]+/>))`.
    Its lookahead needs a byte between the name and the `/`, so a name that runs into `/>` gives its last letter back.
    That letter reads as an unrecognized attribute.
  - 2026-11-12: nishiki needed a byte after the whole name, so it opened a block tag.
    `<a/>` has no letter to give back, and both read it as a block tag.
  - 2026-11-12: `a_vue_name_that_runs_into_its_slash_closes_the_tag` holds the two lines with Shiki's scopes.
    On a box, it fails on the base and passes with the fix.
  - 2026-11-12: Vue's `win` corpus, on a box, 100 documents per seed.
    Before, seed 2 had one, document 78, by 12 bytes.
    After, none differs at any seed.

## 7. Go and LaTeX

- [ ] 7.1 After a struct or interface field that ends in text such as `interface{&=`, Go leaves Shiki's state for the next line.
  → verify: a test holds the case with Shiki's scopes, and no Go document differs at any seed
- [ ] 7.2 A non-ASCII character earlier in a line leaves Go matching the expression Shiki's engine matches.
  → verify: a test holds the case with Shiki's scopes, and no Go document differs at any seed
- [ ] 7.3 On a long line of many words, Go stops an expression where Oniguruma's retry limit stops it.
  → verify: the limit is measured on Shiki's engine and recorded, a test holds the case, and no Go document differs at any seed
- [ ] 7.4 Haskell, Java, Julia, Rust and Scala code in a LaTeX braced argument ends where Shiki ends it.
  → verify: a test holds a `}` inside a string of each, with Shiki's scopes, and no LaTeX document differs at any seed
- [ ] 7.5 Go's and LaTeX's docs list no difference from Shiki.
  → verify: each fixed case leaves "Differences from Shiki", or moves to "Behaviour kept from Shiki"

## 8. The textmate tier

- [ ] 8.1 A tool runs `win` and `mdw` corpora through the TextMate engine with every grammar Shiki loads.
  → verify: it reports, for each language, the documents and bytes that differ
- [ ] 8.2 The textmate tier's differences are measured at seeds 1, 2 and 3.
  → verify: the numbers are recorded here, and each difference gets a task or a fix
- [ ] 8.3 Five textmate-tier grammars embed Objective-C: asciidoc, mdx, org, typst and wikitext.
  In each, Objective-C's `$base` slot tries the embedder's root patterns.
  The TextMate engine answers the slot's query, as Markdown does once task 1.1 lands.
  → verify: at every seed, no document differs in a corpus where each of the five embeds Objective-C

## 9. Bounds

- [ ] 9.1 Each bound a lexer sets itself is listed in its `Language` doc, under "Behaviour kept from Shiki" or "Differences from Shiki".
  → verify: every bound constant in the lexers' sources has its line in the docs

[note]: ../../research/2026-11-11-shiki-exact-gaps.md
[base]: https://github.com/victor-software-house/nishiki/commit/98bb9820
[s2-head]: https://github.com/victor-software-house/nishiki/commit/d36a1da6
