# Tasks

Each fix is measured as design section 1 says: on a box, at seeds 1, 2 and 3.
The [research note][note] holds the numbers each task starts from.

## 1. `$base`

- [ ] 1.1 Objective-C and Objective-C++ try the root patterns of the document that embeds them at each `$base` include, as design section 2 says.
  → verify: at every seed, no document differs in Objective-C's fenced differential, `mdw` and `mdg` corpora.
  Nor does one differ in Objective-C++'s `win` corpus and bare differential, and the oracle test passes.

## 2. The fuzz script

- [ ] 2.1 `oracle/fuzz.ts` refuses `gen` and `mdg` alike for a language with no pieces.
  → verify: both modes give the same answer for C#, Lua, R and Swift, and the pull request records where their `gen` documents came from

## 3. Rust

- [ ] 3.1 Rust reads non-ASCII digits and names with the classes of Shiki's engine, after reading the grammar's exact classes.
  → verify: at every seed, no document differs in Rust's `win` and `mdw` corpora

## 4. Ruby and Mermaid

- [ ] 4.1 A `%w` at the end of a line opens no percent literal, as in Shiki.
  → verify: at every seed, no document differs in Ruby's `win` corpus, and the research note's document 296 matches
- [ ] 4.2 After a here document that runs on past a blank line, the next line's command name gets Shiki's shell scopes.
  → verify: the research note's document 59 matches, and no Ruby `win` document differs at any seed
- [ ] 4.3 In a fence, the text after a line's second here document opener gets the embed Shiki gives it.
  → verify: at every seed, no document differs in Ruby's `mdw` corpus
- [ ] 4.4 Mermaid leaves a backslash after a closed string plain, as Shiki does.
  → verify: at every seed, no document differs in Mermaid's `win` and `mdw` corpora and its differential

## 5. Vue

- [ ] 5.1 Find why Vue's regions end at other lines in a list-item fence.
  → verify: the cause is recorded here, with the rule it comes from
- [ ] 5.2 Vue in a list-item fence ends its regions where Shiki does.
  → verify: at every seed, no document differs in Vue's `mdw` corpus

## 6. Go and LaTeX

- [ ] 6.1 After a struct or interface field that ends in text such as `interface{&=`, Go leaves Shiki's state for the next line.
  → verify: a test holds the case with Shiki's scopes, and no Go document differs at any seed
- [ ] 6.2 A non-ASCII character earlier in a line leaves Go matching the expression Shiki's engine matches.
  → verify: a test holds the case with Shiki's scopes, and no Go document differs at any seed
- [ ] 6.3 On a long line of many words, Go stops an expression where Oniguruma's retry limit stops it.
  → verify: the limit is measured on Shiki's engine and recorded, a test holds the case, and no Go document differs at any seed
- [ ] 6.4 Haskell, Java, Julia, Rust and Scala code in a LaTeX braced argument ends where Shiki ends it.
  → verify: a test holds a `}` inside a string of each, with Shiki's scopes, and no LaTeX document differs at any seed
- [ ] 6.5 Go's and LaTeX's docs list no difference from Shiki.
  → verify: each fixed case leaves "Differences from Shiki", or moves to "Behaviour kept from Shiki"

## 7. The textmate tier

- [ ] 7.1 A tool runs `win` and `mdw` corpora through the TextMate engine with every grammar Shiki loads.
  → verify: it reports, for each language, the documents and bytes that differ
- [ ] 7.2 The textmate tier's differences are measured at seeds 1, 2 and 3.
  → verify: the numbers are recorded here, and each difference gets a task or a fix
- [ ] 7.3 Five textmate-tier grammars embed Objective-C: asciidoc, mdx, org, typst and wikitext.
  In each, Objective-C's `$base` slot tries the embedder's root patterns.
  The TextMate engine answers the slot's query, as Markdown does once task 1.1 lands.
  → verify: at every seed, no document differs in a corpus of Objective-C embedded in each of the five

## 8. Bounds

- [ ] 8.1 Each bound a lexer sets itself is listed in its `Language` doc, under "Behaviour kept from Shiki" or "Differences from Shiki".
  → verify: every bound constant in the lexers' sources has its line in the docs

[note]: ../../research/2026-11-11-shiki-exact-gaps.md
