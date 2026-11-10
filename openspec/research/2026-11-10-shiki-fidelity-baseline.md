# Shiki fidelity baseline

Does nishiki's TextMate engine give Shiki's scopes to every non-whitespace byte of every Shiki grammar's sample, before the public grammar API exists?

It is the baseline for task 3.1 of the [`grammar-api` change][tasks].
The grammars that fail become task 3.2's work.

## Answer

Yes on the inputs, with four grammars that still have a gap.
All 242 languages Shiki 4.6.0 bundles match Shiki on every non-whitespace byte of their input.

- No grammar failed to load.
- The regex compiler refused two patterns in each of four languages: C#, Org, Razor and Typst.
  Their inputs still match, because no input reaches the rule.
- No line reached a limit.
- No language panicked or stalled.

So task 3.2 has four gaps, all from one regex feature.

## Method

1. [`oracle/fidelity.ts`][fidelity-ts] writes, for each language in Shiki 4.6.0's `bundledLanguagesInfo`:
   - every grammar the language loads, as Shiki ships it;
   - Shiki's scopes for the language's input, with Shiki's per-line limits off, in the oracle's fixture format;
   - the engine's tables, from `oracle/engine-tables.ts --json`.
2. A language's input is Shiki's sample, from `shikijs/textmate-grammars-themes` at [8f1828eb][samples].
   Shiki has no sample for four languages.
   Each gets a stand-in, listed with its licence in [`tests/oracle/SOURCES.md`][sources]:

   | Language | Stand-in |
   |:--|:--|
   | `git-commit` | `COMMIT_EDITMSG` from VS Code's colorizer fixtures |
   | `git-rebase` | `git-rebase-todo` from the same fixtures |
   | `html-derivative` | the oracle's `html/h5bp-index` input |
   | `nextflow-groovy` | `test.groovy` from the same fixtures |

3. Each language loads only its own grammar and the grammars it embeds, in Shiki and in the engine.
4. [`every_shiki_grammar_matches_shiki`][check] loads each language's tables through the survey's loader.
   It lexes the input with `Limits::unlimited()`, compares every line, and skips no block.
5. `mise run check:fidelity` runs both steps.
   It ran on a box, a `4x8` build microVM, at [bb582b95][head].
   That commit adds the check on top of [e4609498][base], where the engine lexes through a `Highlighter`.

### Controls

Two runs on the box checked that the comparison can fail:

- One extra scope in the first token of Go's fixture: Go differed at line 1, column 1.
- Python's tables on Ruby's sample: Ruby differed on 420 of 420 bytes.

A third run changed `compare` to use `Limits::default()`, in a copy on the box that was not committed.
All 238 samples measured then still matched, and no line reached a limit.
The four stand-ins did not exist yet in that run.

## Result

[`report.md`][report] holds one row per language: the bytes compared, and the first differing byte where one exists.
Every row is equal.

| Measure | Count |
|:--|--:|
| Languages | 242 |
| Equal on every non-whitespace byte | 242 |
| Tables that failed to build | 0 |
| Languages with a refused pattern | 4 |
| Languages that reached a limit | 0 |
| Panics or stalls | 0 |

## Refused patterns

The regex compiler refuses two patterns in each of four languages.
The report's "Refused patterns" column shows the count and the first.

| Language | Refused patterns |
|:--|:--|
| csharp | `\G(?=(?~\*/)$)` and `^(\s*+)(\*(?!/))?(?=(?~\*/)$)` |
| org | the same two |
| razor | the same two |
| typst | the same two |

- Both need Oniguruma's absent operator, `(?~…)`.
  The compiler gives "unsupported group syntax" for it.
- Both sit in C#'s grammar, at `repository.comment.patterns[1].patterns[0]`, as the `begin` and the `while`.
  That rule lexes the lines inside a `/** */` documentation comment.
  Org, Razor and Typst embed C#'s grammar, so they hold the same rule.
- A refused pattern never matches.
  None of the four inputs holds `/**`, so every input still matches.
- Each language is a gap in task 3.2.
  `KNOWN_REFUSALS` in `fidelity.rs` lists the eight patterns.
  The check fails on any other refusal, and on an entry that no grammar refuses any more, so the list only shrinks.

## Gate through the public API

The check reads the TypeScript compiler's tables, not the public API.

- The function `grammar` in [`fidelity.rs`][check] is the one place that loads a grammar.
- When task 2.1 lands, it calls `Grammar::from_json` on the files in `grammars/`.
- The gate then also depends on task 1.1: the Rust compiler must write the tables the TypeScript compiler writes.

## Not verified

- Inputs past the samples and the four stand-ins.
  A sample covers part of its grammar only.
  `tests/oracle/COVERAGE.md` measures that coverage for the hand-lexed grammars only.
- Colours under a theme.
  The check compares scopes only.
- The scanner-off path (`Scanners::Off`).
- The CI run time.
  On the box, `check:fidelity` took 24 s with a warm build cache, and the oracle took 12 s on a laptop.

## Compute

| Machine | Work | Minutes | Cost |
|:--|:--|--:|--:|
| A `4x8` build box | toolchain install, release builds, five check runs | about 25 | none |
| A laptop | the Bun oracle, three runs | about 3 | none |

[tasks]: https://github.com/victor-software-house/nishiki-archive/blob/ed6bfd7f/openspec/changes/grammar-api/tasks.md
[fidelity-ts]: https://github.com/victor-software-house/nishiki-archive/blob/bb582b95/oracle/fidelity.ts
[samples]: https://github.com/shikijs/textmate-grammars-themes/tree/8f1828ebccc57930cf0adde9b35ae929584fe191/samples
[sources]: https://github.com/victor-software-house/nishiki-archive/blob/bb582b95/tests/oracle/SOURCES.md
[check]: https://github.com/victor-software-house/nishiki-archive/blob/bb582b95/src/textmate/tests/fidelity.rs
[head]: https://github.com/victor-software-house/nishiki-archive/commit/bb582b95
[base]: https://github.com/victor-software-house/nishiki-archive/commit/e4609498
[report]: 2026-11-10-shiki-fidelity-baseline/report.md
