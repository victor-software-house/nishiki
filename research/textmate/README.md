# textmate-proto

A prototype TextMate engine, for the note `docs/research/2026-11-02-textmate-engine.md`.
It is written from scratch, and the nishiki crate takes none of its code until a decision says so.
It is its own Cargo workspace, outside the root workspace.

1. `src/regex.rs`: an Oniguruma-dialect regular-expression parser and backtracking matcher.
2. `src/grammar.rs`: loads Shiki's grammar JSON and links includes across grammars.
3. `src/tokenizer.rs`: the line tokenizer, with begin, end and while rules, captures and the endless-loop guards.
4. `tools/dump-grammars.ts`: writes the grammars the prototype runs under `grammars/` (ignored by Git).
5. `tools/survey.ts`, `tools/inject-survey.ts`, `tools/licences.ts`: the surveys the note cites.

Run from this directory:

```sh
bun tools/dump-grammars.ts
cargo build --release
./target/release/compare --bench 5
```

`compare` needs a large stack and starts a thread for it.
`trace <scope>` prints the tokens of the lines on stdin.
`rx <pattern> <text> <from>` runs one search.
