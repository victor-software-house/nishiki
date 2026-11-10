# tests

nishiki's integration tests and their data.
Unit tests sit beside the code, in each module's `tests.rs`.
The rules for working here are in [AGENTS.md](AGENTS.md).

## `it/`: one test binary

The test `it` builds in `nishiki-tests`, the package of [`Cargo.toml`](Cargo.toml) here, which is never published.
It reads the fixtures below in place, and its archive leaves them out.

| File | Checks |
|:--|:--|
| [`oracle.rs`](it/oracle.rs) | Every fixture gets Shiki's scopes and colours, for every language. |
| [`limits.rs`](it/limits.rs) | Each limit, reached and not reached, through the public API. |
| [`limits/`](it/limits) | Each language's hostile lines finish within the default limits, one file a language. |
| [`fuzz.rs`](it/fuzz.rs) | Every lexer finishes arbitrary lines within the default limits. |
| [`alloc.rs`](it/alloc.rs) | Lexing a line allocates nothing once the arena holds its stacks. |
| [`notices.rs`](it/notices.rs) | `THIRD-PARTY-NOTICES.md` ships, and holds every bundled grammar's licence. |
| [`sorted.rs`](it/sorted.rs) | The lists a new language adds a row to stay sorted. |

## `oracle/`: Shiki's answers

- `<language>/<name>.txt`: an input's text, line by line, with Shiki's scope stack for every token.
- `<language>/<name>.<theme>.colors`: Shiki's colours for the same input, in three themes.
- [`SOURCES.md`](oracle/SOURCES.md): each input's source, commit and licence.
- [`COVERAGE.md`](oracle/COVERAGE.md): how much of each hand-lexed grammar the fixtures exercise.

[`oracle/generate.ts`](../oracle/generate.ts) writes the fixtures; nobody edits them by hand.

## `engine/`: TextMate engine data

- `grammars/`: the Shiki grammars the engine's tests link into tables, as a second oracle for the hand lexers.
  `tables.json` lists what each table links, and [`SOURCES.md`](engine/grammars/SOURCES.md) each grammar's source, commit and licence.
  [`oracle/engine-grammars.ts`](../oracle/engine-grammars.ts) writes them; nobody edits them by hand.
- `absent/`: Oniguruma's answers for patterns with the absent operator `(?~…)`, which the engine's regex tests read.
- `case-folds/`: Oniguruma's case-fold answers, which the engine's fold tests read.
