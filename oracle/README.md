# oracle

Bun scripts that run Shiki, the reference nishiki matches, and write what the tests and the generated tables compare against.
They need Bun and the network, so they run by hand or in their own workflows, never in the offline `mise run verify`.
The rules for working here are in [AGENTS.md](AGENTS.md).

## The fixtures

- [`inputs.ts`](inputs.ts): the inputs, each pinned to a commit.
  [`tests/oracle/SOURCES.md`](../tests/oracle/SOURCES.md) lists their licences.
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
- [`fence-stops.ts`](fence-stops.ts): checks that `Language::pops_fence` names every Markdown fence grammar whose root can stop without advancing.
  `mise run check:rule-grammars` runs it.

## Generated tables

Each writes Rust source under `src/` from Shiki's grammars or from Oniguruma.
The file it writes marks itself as generated in its first line; edit the script, never the output.

| Script | Writes |
|:--|:--|
| `<language>-grammar.ts` | `src/<language>/scope.rs` and `grammar.rs` for a language on the rule engine. [`rule-grammar.ts`](rule-grammar.ts) holds what they share, and [`rule-grammar.test.ts`](rule-grammar.test.ts) tests its pattern analysis. |
| `<language>-words.ts` | `src/<language>/words.rs`: the words of a grammar's large alternations. [`alternation.ts`](alternation.ts) expands them. |
| [`engine-tables.ts`](engine-tables.ts) | The tables of a language on nishiki's TextMate engine, from the two lists below the table. |
| [`classes.ts`](classes.ts) | `src/classes/table.rs`: Oniguruma's character classes. |
| [`unicode-tables.ts`](unicode-tables.ts) | `src/textmate/regex/unicode.rs`: Oniguruma's Unicode sets. |
| [`case-folds.ts`](case-folds.ts) | `src/textmate/regex/folds.rs`: Oniguruma's case folds. [`case-fold-probe.ts`](case-fold-probe.ts) writes the cases that test them. |

`engine-tables.ts` reads two lists:

- [`licences.ts`](licences.ts) records each grammar's licence, read at its source;
- [`hand-lexed.ts`](hand-lexed.ts) lists the hand-lexed languages a table can hand a region to.
