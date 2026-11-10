# oracle

Bun scripts that run Shiki, the reference nishiki matches, and write what the tests and the generated tables compare against.
They need Bun and the network, so they run by hand or in their own workflows, never in the offline `mise run verify`.
The repository's [`package.json`](../package.json) and [`bun.lock`](../bun.lock) pin the npm packages they import, down to every dependency.
The rules for working here are in [AGENTS.md](AGENTS.md).

## The fixtures

- [`inputs.ts`](inputs.ts): the inputs, each pinned to a commit.
  [`tests/oracle/SOURCES.md`](../tests/oracle/SOURCES.md) lists their licences.
- [`sources.ts`](sources.ts): reads each third-party file from the public mirror [victor-software-house/nishiki-sources](https://github.com/victor-software-house/nishiki-sources), and fills and checks that mirror.
- [`inputs/`](inputs): inputs written for nishiki, where no third-party file covers a case.
- [`grammars.ts`](grammars.ts): the grammars every script loads into Shiki.
- [`fixture.ts`](fixture.ts): the fixture format.
- [`generate.ts`](generate.ts): writes Shiki's scopes and colours for every input into `tests/oracle/<language>/`.
  `mise run oracle` runs it.

## Checks past the fixtures

- [`fuzz.ts`](fuzz.ts): writes a corpus of mutated and generated documents, bare and in Markdown fences.
  [`fuzz/`](fuzz) has what it draws on for each language.
- [`dump.ts`](dump.ts): writes a corpus of held-out source files.
- [`differential.ts`](differential.ts): writes random documents for one language, in the fixture format.
- [`onig.ts`](onig.ts): compares a language's hand matchers with Oniguruma, one expression at a time.
- [`coverage.ts`](coverage.ts): writes [`tests/oracle/COVERAGE.md`](../tests/oracle/COVERAGE.md), how much of each hand-lexed grammar the fixtures exercise.
  [`coverage-waivers.ts`](coverage-waivers.ts) lists the waivers and the complete grammars.
- [`fidelity.ts`](fidelity.ts): writes every grammar Shiki bundles, and Shiki's scopes for its sample and for a few extra inputs, into `target/fidelity/`.
  `mise run check:fidelity` compares the engine with Shiki there, and the Shiki fidelity workflow runs it on every pull request.
- [`engine-tables.ts`](engine-tables.ts): writes the engine's tables for Shiki's grammars as JSON, the oracle for the Rust compiler.
  `link_survey` in `src/textmate/compile/tests.rs` checks that the compiler writes the same tables.
  [`hand-lexed.ts`](hand-lexed.ts) lists the hand-lexed languages a table can hand a region to.
- [`fence-stops.ts`](fence-stops.ts): checks that `Language::pops_host` names every Markdown fence grammar whose root can stop without advancing.
  `mise run check:rule-grammars` runs it.

## Generated tables

Each writes Rust source under `src/` from Shiki's grammars or from Oniguruma.
The file it writes marks itself as generated in its first line; edit the script, never the output.

| Script | Writes |
|:--|:--|
| `<language>-grammar.ts` | `src/<language>/scope.rs` and `grammar.rs` for a language on the rule engine. [`rule-grammar.ts`](rule-grammar.ts) holds what they share, and [`rule-grammar.test.ts`](rule-grammar.test.ts) tests its pattern analysis. |
| `<language>-words.ts` | `src/<language>/words.rs`: the words of a grammar's large alternations. [`alternation.ts`](alternation.ts) expands them. |
| [`engine-grammars.ts`](engine-grammars.ts) | `tests/engine/grammars/`: the Shiki grammars the engine's tests link into tables, with what each table links and where each grammar comes from. |
| [`classes.ts`](classes.ts) | `src/classes/table.rs`: Oniguruma's character classes. |
| [`unicode-tables.ts`](unicode-tables.ts) | `src/textmate/regex/unicode.rs`: Oniguruma's Unicode sets. |
| [`absent-probe.ts`](absent-probe.ts) | `tests/engine/absent/`: Oniguruma's answers for patterns with the absent operator `(?~…)`, which test how `src/textmate/regex/parse.rs` reads it. |
| [`case-folds.ts`](case-folds.ts) | `src/textmate/regex/folds.rs`: Oniguruma's case folds. [`case-fold-probe.ts`](case-fold-probe.ts) writes the cases that test them. |

Both regex probes read each Oniguruma answer with [`oniguruma-answer.ts`](oniguruma-answer.ts).

[`licences.ts`](licences.ts) records each grammar's licence, read at its source.
`engine-grammars.ts` refuses a grammar that list refuses or lacks.

## Shared helpers

- [`dedent.ts`](dedent.ts): `dedent`, a template tag for multiline text written at the indentation of the code that builds it.
  [`dedent.test.ts`](dedent.test.ts) tests it.

## Tests

`mise run test:scripts` runs every `*.test.ts` under `oracle/` and `bench/`.
