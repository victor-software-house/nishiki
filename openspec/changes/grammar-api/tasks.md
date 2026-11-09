# Tasks

## 1. Compiler

- [ ] 1.1 A Rust compiler from grammar JSON to the engine's tables.
  → verify: its tables equal `oracle/engine-tables.ts --json` for every grammar Shiki 4.6.0 bundles
- [ ] 1.2 Retire the TypeScript generator for shipped tables, and keep it as the compiler's oracle.
  → verify: `check:features` and Identity pass with the Rust compiler's tables

## 2. Public API

- [ ] 2.1 `Grammar`, `Grammars`, `GrammarError` and `Grammar::from_json`, with process-wide interning keyed on the full source and the linked includes.
  → verify: a unit test loads one grammar twice and gets one handle, and the same source with other includes gets another
- [ ] 2.2 `Highlighter::for_grammar` and its limits variant → verify: a doctest highlights a grammar loaded from JSON
- [ ] 2.3 Includes across grammars and hand-offs to hand lexers.
  → verify: an HTML grammar loaded from JSON hands its `<style>` to the CSS lexer, and the fixture matches Shiki
- [ ] 2.4 `to_bytes` and `from_bytes`, with a versioned header.
  → verify: a round trip gives the same output; bytes of another version, truncated bytes and an out-of-range index each fail with `GrammarError`

## 3. Fidelity

- [ ] 3.1 A CI job loads every Shiki 4.6.0 grammar through the public API and compares Shiki's sample.
  → verify: the job passes with no differing non-whitespace byte
- [ ] 3.2 One task per grammar that fails 3.1, added as each is found → verify: that grammar's sample matches

## 4. Speed and docs

- [ ] 4.1 A bench row for compiling and loading a grammar, from JSON and from bytes → verify: the table in `bench/README.md`
- [ ] 4.2 Crate docs: an "Any language" section, a build-script example, and the limits a grammar runs under.
  → verify: the docs.rs preview builds, and every example runs as a doctest
- [ ] 4.3 `AGENTS.md` invariant 4 and the README describe grammars callers bring → verify: review
