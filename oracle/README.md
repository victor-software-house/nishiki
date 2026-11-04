# oracle

Bun scripts that run Shiki, the reference nishiki matches, and write what the tests and the generated tables compare against.
They need Bun and the network, so they run by hand or in their own workflows, never in the offline `mise run verify`.
The rules for working here are in [AGENTS.md](AGENTS.md).

## The fixtures

| Script | Writes or does |
|:--|:--|
| [`inputs.ts`](inputs.ts) | The inputs, each pinned to a commit. [`tests/oracle/SOURCES.md`](../tests/oracle/SOURCES.md) lists their licences. |
| [`grammars.ts`](grammars.ts) | The grammars every script loads into Shiki. |
| [`fixture.ts`](fixture.ts) | The fixture format. |
| [`generate.ts`](generate.ts) | Shiki's scopes and colours for every input, into `tests/oracle/<language>/`. Run by `mise run oracle`. |
| [`inputs/`](inputs) | Inputs written for nishiki, where no third-party file covers a case. |

## Checks past the fixtures

| Script | Writes or does |
|:--|:--|
| [`fuzz.ts`](fuzz.ts) | A corpus of mutated and generated documents, bare and in Markdown fences. [`fuzz/`](fuzz) holds the pieces for one language each. |
| [`dump.ts`](dump.ts) | A corpus of held-out source files. |
| [`differential.ts`](differential.ts) | Random documents for one language, in the fixture format. |
| [`onig.ts`](onig.ts) | Compares a language's hand matchers with Oniguruma, one expression at a time. |
| [`coverage.ts`](coverage.ts) | How much of each hand-lexed grammar the fixtures exercise, into [`tests/oracle/COVERAGE.md`](../tests/oracle/COVERAGE.md); [`coverage-waivers.ts`](coverage-waivers.ts) lists the waivers and the complete grammars. |

## Generated tables

Each writes Rust source under `src/` from Shiki's grammars or from Oniguruma.
The file it writes says so in its first line; edit the script, never the output.

| Script | Writes |
|:--|:--|
| `<language>-grammar.ts` | `src/<language>/scope.rs` and `grammar.rs` for a language on the rule engine. [`rule-grammar.ts`](rule-grammar.ts) holds what they share. |
| `<language>-words.ts` | `src/<language>/words.rs`: the words of a grammar's large alternations. [`alternation.ts`](alternation.ts) expands them. |
| [`engine-tables.ts`](engine-tables.ts) | The tables of a language on nishiki's TextMate engine. [`licences.ts`](licences.ts) records each grammar's licence, read at its source. |
| [`classes.ts`](classes.ts) | `src/classes/table.rs`: Oniguruma's character classes. |
| [`unicode-tables.ts`](unicode-tables.ts) | `src/textmate/regex/unicode.rs`: Oniguruma's Unicode sets. |
| [`case-folds.ts`](case-folds.ts) | `src/textmate/regex/folds.rs`: Oniguruma's case folds. [`case-fold-probe.ts`](case-fold-probe.ts) writes the cases that test them. |
