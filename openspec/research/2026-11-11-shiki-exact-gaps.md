# Shiki-exact gaps

Which languages still give other scopes than Shiki, on main at [5c1daf55][main]?

Shiki-exact output is a release gate for 0.1.1.
This note lists the gaps that the [`shiki-exact-gaps` change][change] closes.

## Answer

Five hand lexers differ from Shiki on mutated or fenced text.
The textmate tier differs nowhere that CI measures.
CI gives it one input per language.

| Language | Corpus | Documents that differ | Bytes that differ |
|:--|:--|--:|--:|
| Objective-C | differential, in Markdown fences | 89 of 100 | 63,074 |
| Objective-C | fixture windows, in Markdown fences (`mdw`) | 87 of 100 | 106,743 |
| Objective-C | generated documents, in Markdown fences (`mdg`) | 87 of 100 | 171,077 |
| Vue | `mdw` | 6 of 100 | 2,023 |
| Rust | fixture windows (`win`) | 3 of 300 | 6 |
| Rust | `mdw` | 5 of 100 | 30 |
| Ruby | `win` | 2 of 300 | 241 |
| Ruby | `mdw` | 1 of 100 | 1 |
| Mermaid | `win` | 1 of 300 | 1 |

Every other corpus has no differing document.
Objective-C's bare corpora have none either.

## Method

1. The Shiki fidelity job's latest run on main is [42930359016][fidelity-run].
   It ran in the merge group of nishiki-archive#752, at 5c1daf55.
   Its log reports 243 of 243 inputs equal and no refused pattern.
2. The job summary is not in the log.
   So `mise run check:fidelity` ran again on a box at 5c1daf55, with nishiki-sources at the job's pinned commit.
   Its report is [`fidelity-report.md`][report].
3. On the same box, Shiki 4.6.0 tokenized these corpora at seed 1.
   Its grammars are those of `@shikijs/langs` 4.6.0, which the root `bun.lock` pins.
   Its per-line limits were off, as [`oracle/fixture.ts`][fixture] sets them.
   The corpora:
   - for each of the 73 languages with oracle fixtures, 300 `win` documents and 100 `mdw` documents from [`oracle/fuzz.ts`][fuzz];
   - for each of the 21 languages with a file in `oracle/fuzz/`, 300 `gen` documents and 100 `mdg` documents;
   - for each of the 39 languages of [`oracle/differential.ts`][differential], 100 documents of 30 lines, bare and in a Markdown fence.
4. `the_corpus_matches_shiki` in [`tests/it/oracle.rs`][oracle-test] compared each corpus with nishiki, built with every feature.
   [`corpora.tsv`][corpora] holds its total for each of the 262 corpora.
   [`differences.md`][differences] holds the first two differing documents of each corpus that differs.

## Findings

### The fidelity report

All 243 rows read `equal`.
The columns Bytes that differ, First difference, Shiki, nishiki, Refused patterns and Limits reached are empty on every row.

### Objective-C in Markdown fences

- Shiki's `objective-c` grammar includes `$base` 21 times, and its `objective-cpp` grammar 53 times.
  No other grammar of a hand lexer includes it.
  This count was read in Shiki 4.6.0's `@shikijs/langs` files.
- In a fence, Shiki puts Markdown scopes inside Objective-C's, such as `meta.paragraph.markdown`.
  So Shiki resolves `$base` to the document's root grammar, Markdown.
- [`oracle/rule-generator.ts`][generator] resolves `$base` to the grammar's own root (line 427).
- The TextMate engine resolves `$base` to the root of the grammar it links ([`link.rs`][link], line 444).
- Objective-C++'s fenced corpora have no differing document.
  Shiki's Markdown has no Objective-C++ fence: its `mm` fence takes Objective-C.
  So those fences are plain text in both.

### Vue in list-item fences

Each differing document has its fence inside a list item.
Shiki ends a `<template>` or `<script>` region on lines where nishiki keeps it open.
The closing line of a fence nested in the Vue text differs too.
The cause is unchecked.

### Rust and non-ASCII text

- Shiki scopes the Arabic-Indic digit `٣` as `constant.numeric.decimal.rust`.
- Shiki scopes a name that holds `🦀` as a variable or type name.
- nishiki leaves both plain.
- The grammar's exact classes are unchecked.

### Ruby

- A `%w` at the end of a line opens a percent literal in nishiki that runs over the next lines.
  Shiki opens none (239 bytes).
- A here document whose terminator reads `SHELL\` runs on, then a blank line, then `sh = <<~'SH'`.
  Shiki gives `sh` the shell command name scopes, and nishiki does not (2 bytes).
- In a fence, after `pair = [<<~YAM>L, <<~SQL]`, Shiki puts the `]` inside the SQL embed.
  nishiki keeps it as Ruby (1 byte).

### Mermaid

In `click B "https://example.com"\`, nishiki scopes the last `\` as part of the string.
Shiki leaves it plain.

### The fuzz script's modes

`oracle/fuzz.ts` refused `mdg` for C#, Lua, R and Swift: "needs pieces".
It wrote 300 `gen` documents for each of them.
Its guard tests `mode.endsWith("g")`, which `gen` fails.
Where those `gen` documents come from is unchecked.

### Documented differences the corpora did not reach

- Go's `Language` docs list three cases that differ from Shiki.
- LaTeX's docs list one under "Differences from Shiki": code in a braced argument ends at its first `}`.

## Not verified

- One seed per corpus, with 100 to 300 documents.
  Rarer differences can remain.
- The textmate tier ran only on its one input per language, with no mutated text.
- The four languages above got no `mdg` corpus.

## Compute

One build box on the build host, 8 vCPUs and 16 GiB, for about 25 minutes.
It cost nothing beyond the build host.

[main]: https://github.com/victor-software-house/nishiki-archive/commit/5c1daf5520706d9b77991157377c830b7b87d469
[change]: ../changes/shiki-exact-gaps/proposal.md
[fidelity-run]: https://github.com/victor-software-house/nishiki-archive/actions/runs/42930359016
[report]: 2026-11-11-shiki-exact-gaps/fidelity-report.md
[corpora]: 2026-11-11-shiki-exact-gaps/corpora.tsv
[differences]: 2026-11-11-shiki-exact-gaps/differences.md
[fuzz]: https://github.com/victor-software-house/nishiki-archive/blob/5c1daf5520706d9b77991157377c830b7b87d469/oracle/fuzz.ts
[fixture]: https://github.com/victor-software-house/nishiki-archive/blob/5c1daf5520706d9b77991157377c830b7b87d469/oracle/fixture.ts#L36
[differential]: https://github.com/victor-software-house/nishiki-archive/blob/5c1daf5520706d9b77991157377c830b7b87d469/oracle/differential.ts
[oracle-test]: https://github.com/victor-software-house/nishiki-archive/blob/5c1daf5520706d9b77991157377c830b7b87d469/tests/it/oracle.rs
[generator]: https://github.com/victor-software-house/nishiki-archive/blob/5c1daf5520706d9b77991157377c830b7b87d469/oracle/rule-generator.ts#L427
[link]: https://github.com/victor-software-house/nishiki-archive/blob/5c1daf5520706d9b77991157377c830b7b87d469/src/textmate/compile/link.rs#L444
