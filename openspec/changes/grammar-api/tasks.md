# Tasks

## 1. Compiler

- [ ] 1.1 A Rust compiler from grammar JSON to the engine's tables.
  → verify: its tables equal `oracle/engine-tables.ts --json` for every grammar Shiki 4.6.0 bundles
- [ ] 1.2 Retire the TypeScript generator for shipped tables, and keep it as the compiler's oracle.
  → verify: `check:features` and Identity pass with the Rust compiler's tables

## 2. Public API

- [x] 2.1 `Grammar`, `Grammars`, `GrammarError` and `Grammar::from_json`, with process-wide interning keyed on the full source.
  → verify: a unit test loads one grammar twice and gets one handle
  2026-11-10 UTC: `loading_one_grammar_twice_gives_one_handle` in `src/grammar/tests.rs` passes on a box, in debug.
- [x] 2.2 `Highlighter::for_grammar` and its limits variant.
  → verify: a doctest highlights a grammar loaded from JSON
  2026-11-10 UTC: the "Any language" doctest in `src/lib.rs` passes on a box.
- [ ] 2.3 Includes across grammars, linked through the `Grammars` set when the highlighter is built, and hand-offs to hand lexers.
  → verify:
  - an HTML grammar loaded from JSON hands its `<style>` to the CSS lexer, and the fixture matches Shiki;
  - two grammars that include each other, loaded in either order, both match Shiki.
- [x] 2.4 `to_bytes` and `from_bytes`, with a versioned header.
  → verify:
  - a round trip gives the same output;
  - bytes of another version, truncated bytes and an out-of-range index each fail with `GrammarError`.
  2026-11-10 UTC: in `src/grammar/tests.rs`, these pass on a box, in debug:
  - `a_grammar_from_bytes_equals_the_grammar_from_json`;
  - `bytes_of_another_version_fail`;
  - `every_truncation_fails`;
  - `a_rule_index_out_of_range_fails`.

## 3. Fidelity

- [ ] 3.1 A CI job loads every Shiki 4.6.0 grammar through the public API and compares Shiki's sample.
  → verify: the job passes with no differing non-whitespace byte
  - 2026-11-10: the Shiki fidelity workflow runs the check through the survey's loader until 2.1 lands.
    All 242 languages match on their input, and four refuse two patterns each ([baseline][fidelity-baseline]).
- [ ] 3.2 One task per grammar that fails 3.1, added as each is found → verify: that grammar's sample matches
  - [x] 3.2.1 csharp: the regex compiler refuses two patterns that use Oniguruma's absent operator, `(?~…)`, in C#'s rule for `/** */` documentation comments ([baseline][fidelity-baseline]).
    → verify: both compile, and their entries leave `KNOWN_REFUSALS` in `src/textmate/tests/fidelity.rs`
    - 2026-11-10: the parser reads `(?~X)` as `(?:(?!X)\O)*`, as Shiki's Oniguruma answers.
      Both patterns compile, and `KNOWN_REFUSALS` is empty.
      The fidelity check also runs C# on `oracle/inputs/csharp-coverage.cs`, which holds `/**` comments: 243 of 243 inputs match.
  - [x] 3.2.2 org: the same two patterns, from the C# grammar it embeds.
    → verify: as 3.2.1
    - 2026-11-10: as 3.2.1.
  - [x] 3.2.3 razor: the same two patterns, from the C# grammar it embeds.
    → verify: as 3.2.1
    - 2026-11-10: as 3.2.1.
  - [x] 3.2.4 typst: the same two patterns, from the C# grammar it embeds.
    → verify: as 3.2.1
    - 2026-11-10: as 3.2.1.

## 4. Speed and docs

- [ ] 4.1 A bench row for compiling and loading a grammar, from JSON and from bytes → verify: the table in `bench/README.md`
- [ ] 4.2 Crate docs: an "Any language" section, a build-script example, and the limits a grammar runs under.
  → verify: the docs.rs preview builds, and every example runs as a doctest
- [ ] 4.3 `AGENTS.md` invariant 4 and the README describe grammars callers bring → verify: review

[fidelity-baseline]: ../../research/2026-11-10-shiki-fidelity-baseline.md
