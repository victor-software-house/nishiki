# Proposal

## Motivation

Output that matches Shiki exactly is a release gate for 0.1.1.
CI's oracle test and the Shiki fidelity job pass on main.
Mutated and fenced text still finds differences.

The [research note][note] measured 262 corpora at 5c1daf55.
Five hand lexers differ on some of them: Objective-C, Vue, Rust, Ruby and Mermaid.
Go and LaTeX list differences from Shiki in their own docs.
Two parts of the measurement have gaps of their own:

- the textmate tier gets one input per language, with no mutated text;
- `oracle/fuzz.ts` refuses `mdg` for four languages whose `gen` mode runs.

## Changes

- Objective-C resolves `$base` to the root of the document that embeds it, as Shiki does.
  So does every generated lexer whose grammar includes `$base`.
- `oracle/fuzz.ts` checks both modes that need pieces in the same way.
- Rust reads non-ASCII digits and names with the classes Shiki's engine uses.
- Svelte's `{#each}` with a `$` index and its `<script lang="` case give Shiki's scopes.
  A third document that differs at seed 2 gets its language found.
- Ruby's three cases and Mermaid's trailing backslash give Shiki's scopes.
- Vue in a list-item fence ends its regions where Shiki does.
- Go's three documented cases and LaTeX's braced-argument embeds give Shiki's scopes.
- A fuzz run sends mutated and fenced text through the TextMate engine with Shiki's grammars.

## Out of scope

The bounds a lexer sets itself, such as C++'s nesting depths, stay as they are.
Each is a per-line limit that the next minor release moves into `Limits` (root guide, "Limits").
They get no fidelity task here (lead, 2026-11-11).
Their `Language` docs must still list them.

## Capabilities

### Modified Capabilities

None: the fixes bring nishiki to the `shiki-fidelity` spec as written.

## Impact

- `src/objective_c/`, `src/objective_cpp/` and their generators.
- `src/rust/`, `src/svelte/`, `src/ruby/`, `src/mermaid/`, `src/vue.rs`, `src/go/` and `src/latex/`.
- `oracle/fuzz.ts`, and a corpus run for the TextMate engine.
- The `Language` docs in `src/language.rs`.

[note]: ../../research/2026-11-11-shiki-exact-gaps.md
