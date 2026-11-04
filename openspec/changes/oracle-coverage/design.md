# Design

How the oracle's coverage of each hand-lexed grammar is measured, reported and kept.
See [proposal.md](proposal.md) for why.

## Context

- The oracle writes one fixture per pinned input ([`oracle/generate.ts`][generate]).
  Each fixture holds the input's lines and Shiki's scope stacks.
  The [oracle test][oracle-test] fails when nishiki gives any non-whitespace byte a different stack.
- [`oracle/fuzz.ts`][fuzz] writes a mutated corpus in the same format.
  The ignored test `the_corpus_matches_shiki` compares it, run by hand.
- [`oracle/onig.ts`][onig] compares a lexer's matchers with Oniguruma, one expression at a time.
  It applies to C++, C#, Lua and Swift.
  Their matchers follow the grammar's patterns one for one.
- The hand lexers on main, and the Shiki grammars they follow:

  | Lexer | Grammars |
  |:--|:--|
  | C | `c` |
  | C++ | `cpp`, `cpp-macro` |
  | C# | `csharp` |
  | CSS, Less, SCSS | `css`, `less`, `scss` |
  | diff, Dockerfile, INI, Make | `diff`, `docker`, `ini`, `make` |
  | ERB | `erb` |
  | Go, Java, Kotlin | `go`, `java`, `kotlin` |
  | GraphQL | `graphql` |
  | HTML, html-derivative, XML | `html`, `html-derivative`, `xml` |
  | JSON | `json`, `jsonc`, `jsonl` |
  | Lua, Python, Ruby, Rust, Swift | `lua`, `python`, `ruby`, `rust`, `swift` |
  | Markdown | `markdown` |
  | shell | `shellscript` |
  | SQL, TOML, YAML | `sql`, `toml`, `yaml` |
  | TypeScript | `typescript`, `javascript`, `jsx`, `tsx` |

  The languages on nishiki's TextMate engine are left out.
  The engine runs their grammar itself.

## 1. A first measurement

A throwaway script on 2026-11-04 tagged every expression of one grammar.
It tokenized that language's fixtures through Shiki, as section 3 describes.
It counted every expression in the grammar, reachable or not.
Its capture count was rough.

| Grammar | Fixture lines | Expressions matched | Captures matched |
|:--|--:|--:|--:|
| `csharp` | 2,309 | 224 of 547 | 117 of 525 |
| `swift` | 8,811 | 264 of 448 | 136 of 448 |
| `python` | 13,467 | 163 of 352 | 111 of 372 |
| `go` | 11,632 | 201 of 336 | 101 of 250 |
| `java` | 8,068 | 147 of 214 | 49 of 134 |
| `rust` | 3,522 | 70 of 95 | 50 of 99 |
| `sql` | 4,930 | 43 of 75 | 34 of 79 |
| `kotlin` | 6,132 | 57 of 65 | 24 of 35 |
| `toml` | 1,924 | 40 of 62 | 16 of 58 |
| `yaml` | 1,913 | 31 of 62 | 14 of 55 |
| `diff` | 611 | 10 of 16 | 10 of 23 |
| `ini` | 1,289 | 14 of 15 | 6 of 12 |

Long fixtures do not imply coverage.
Python's 13,467 lines match under half of its grammar.

## 2. What a unit is

Coverage counts units of a grammar:

1. **An expression**: each `match`, `begin`, `end` and `while` regex.
2. **A capture**: each group of an expression that the grammar names or gives patterns.
   `beginCaptures`, `endCaptures` and `whileCaptures` belong to their expression.
   A rule's `captures` applies to its `begin` and its `end` both, as in vscode-textmate.
   It gives a unit for each.

A unit is **reachable** in two cases:
the grammar's root reaches it through `include`s, or another loaded grammar includes it by name.
Unreachable units are listed apart and left out of the share.

A unit is **exercised** when, while Shiki tokenizes the corpus:

1. an expression is the match Shiki's scanner returns at least once, a zero-width match included;
2. a capture's group takes part in such a match and holds at least one non-whitespace byte.
   The oracle test compares non-whitespace bytes only.
   A capture of blanks alone checks nothing.

Each unit is named by its path in the grammar, such as `#attributes.patterns[1].begin` or `#import.beginCaptures[2]`.
A path is stable while Shiki's version stays pinned.

## 3. How it is measured

The measure runs on Shiki's side, in Bun, with no Rust build:

1. `oracle/coverage.ts` loads the same grammars as `generate.ts`.
   It prefixes every expression of a hand-lexed grammar with an Oniguruma comment, `(?#u<n>)`, where `n` numbers the unit.
   Oniguruma [ignores a comment][onig-re].
   vscode-textmate rewrites `\G` and back-references, but keeps the prefix.
2. It passes Shiki an [engine][shiki-engines] that wraps Shiki's own Oniguruma engine.
   Each scanner result names the source it matched.
   The wrapper reads `n` from that source, along with the groups that took part.
3. It tokenizes the text of every fixture.
   Each fixture records its text line by line, so nothing is fetched.
4. It checks that the tags change nothing.
   The scope stacks it gets must equal every fixture's, byte for byte.
   A difference fails the run.
5. It writes a fuzz corpus with `oracle/fuzz.ts` at fixed seeds into a temporary directory.
   It measures that corpus the same way.

HTML, html-derivative and ERB declare injections.
There vscode-textmate scans the rule list and each injection at the same position.
It keeps one result.
The wrapper groups the scans made at one position.
It keeps the winner by vscode-textmate's rule: the earlier start, then the injection's priority.

Alternatives considered:

- **Count matcher hits in an instrumented nishiki build.**
  Only the rule-engine lexers have a pattern table to count against.
  TypeScript, Markdown, shell and the other early lexers do not.
  A nishiki hit also does not prove Shiki took the same path.
- **Trace vscode-textmate's own rule ids.**
  Shiki's bundle does not expose them, and its debug output is not a stable interface.
- **Walk the grammar with a tokenizer of our own.**
  That copies an engine, which the [repository's first invariant][agents] rules out.
  Our tokenizer could also disagree with Shiki.

## 4. The report

`mise run oracle:coverage` writes `tests/oracle/COVERAGE.md`.
A second run with the same inputs writes the same bytes.

1. A summary table, one row per hand-lexed grammar.
   Its columns:
   - lexer;
   - grammar;
   - expressions exercised by fixtures, as a count and a percent;
   - captures exercised by fixtures, likewise;
   - units waived;
   - units reached only by the fuzz corpus;
   - whether the grammar is complete.
2. One section per grammar, listing every reachable unit that no fixture exercises.
   Each entry gives:
   - its path;
   - the first 60 characters of its regex;
   - `fuzz`, if the fuzz corpus reaches it;
   - the waiver's reason, if it has one.
3. A last section listing the unreachable units per grammar.

The fuzz column is information only.
The fuzz corpus is not compared in CI.
A unit it alone reaches has unchecked scopes until someone runs `the_corpus_matches_shiki`.

## 5. Target and gate

**Target.**
A grammar is complete when every reachable unit is exercised by a fixture, or waived.
The goal is every hand-lexed grammar complete.

**Waivers.**
`oracle/coverage-waivers.ts` lists each waived unit by grammar and path, with a reason.
A waiver is for a unit no document can reach, such as a branch that a previous rule always takes first.
Where the lexer has `onig.ts` support, the waiver names the probe case that checks the matcher instead.
A waiver for a unit that the fixtures do exercise fails the run.
Waivers therefore do not outlive their reason.

**Gate.**
`mise run check:oracle-coverage` writes the report again and fails:

1. when it differs from the committed report;
2. when a grammar listed complete in `oracle/coverage-waivers.ts` has an unexercised unit without a waiver.

`.github/workflows/coverage.yml` runs the check on pull requests that touch `oracle/`, `tests/oracle/` or the check itself.
It follows [`tables.yml`][tables]: Bun alone, on a 2x4 runner, outside `mise run verify`.
Bun fetches Shiki from npm, and `verify` runs offline.
Coverage depends only on grammars and inputs.
A change to a lexer alone cannot move it, so `src/` does not trigger the check.

A grammar joins the complete list in the pull request that closes its last gap.
From then on, a change that loses a unit fails until it adds an input or a waiver.

## 6. Closing a gap

For each unexercised unit, in this order:

1. **A third-party input**: a file pinned to a commit.
   Its licence must be found at its source and permit it.
   It is recorded in `tests/oracle/SOURCES.md`.
   Held-out inputs are preferred where the language already has development ones.
2. **An input written for nishiki**: a short file under `oracle/inputs/`, MIT OR Apache-2.0 as nishiki.
   It groups related gaps, as `ruby-heredocs.rb` does.
3. **A waiver with a probe**: only for a unit no document can reach.

A new fixture enters the oracle test, so it may expose a lexer bug.
The input then lands with the fix, in one pull request, with `identity: changes <language>` when output changes.

The work goes language by language, largest gap first among the most used lexers.
Each language is one pull request, or a short stack when its gaps are many.

## Risks / Trade-offs

- [A tag could change how Shiki matches] → Step 4 of section 3 compares every fixture's scopes with the tags in place, on every run.
- [Shiki's scanner returns a match that vscode-textmate then drops] → Only injections compete for one position on the hand-lexed grammars, and the wrapper settles them as vscode-textmate does. The script's own test covers a document where each side wins.
- [A Shiki upgrade renames every path] → Paths are regenerated with the fixtures in the upgrade's pull request. Waivers are keyed by path. The check names any waiver that no longer resolves.
- [Exercised does not mean every branch of an alternation is checked] → Units stop at whole expressions and named groups. Branches inside one regex are what `onig.ts` probes cover.
- [Written inputs drift from real code] → Third-party inputs come first, and written inputs stay short and grouped by gap.

## Migration Plan

1. Land the script, the report and the workflow, with no grammar complete.
   The gate then only keeps the report current.
2. Close gaps language by language.
   Each closing pull request adds its grammar to the complete list.

Rollback is deleting the workflow.
The report and inputs stay useful without it.

[generate]: ../../../oracle/generate.ts
[oracle-test]: ../../../tests/it/oracle.rs
[fuzz]: ../../../oracle/fuzz.ts
[onig]: ../../../oracle/onig.ts
[tables]: ../../../.github/workflows/tables.yml
[agents]: ../../../AGENTS.md
[onig-re]: https://github.com/kkos/oniguruma/blob/f95747b462de672b6f8dbdeb478245ddf061ca53/doc/RE#L262
[shiki-engines]: https://github.com/shikijs/shiki/blob/167a38fea19cfa92b760ba36e157589ca5801274/docs/guide/regex-engines.md
