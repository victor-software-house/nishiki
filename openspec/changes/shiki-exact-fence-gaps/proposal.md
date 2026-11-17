# Proposal

## Motivation

Output that matches Shiki exactly is a release gate.
Task 6.4 of [`shiki-exact-gaps`][gaps] opens a `vue` fence where a line starts in any fence's body.
Its sweep and corpora found twelve more cases that differ from Shiki.
`shiki-exact-gaps` has no room left in its task budget, so they get a change of their own.

## Changes

- Markdown opens a `vue` fence in a paragraph's line after four spaces.
  A `vue` fence in a Vue body in a quote's fence sits on Shiki's scopes.
- A `vue` fence in C#, PHP and Ruby fences sits where Shiki puts it.
  The fence ends where Shiki ends it.
- Vue reads an attribute name that holds a `.` as a directive.
- Svelte opens a block in a TypeScript template string.
- Lua in LaTeX's `\verb` gives its words Shiki's scopes.
- Bat, Python and YAML give four cases in fences Shiki's scopes.
- Blade's PHP and PHP read Oniguruma's Unicode `\s` after a type and before a heredoc's label.
- Twelve languages with hand lexers open a `vue` fence where one of their rules begins.

## Out of scope

The cases `shiki-exact-gaps` already holds, such as its tasks 6.5 to 6.8.

## Capabilities

### Modified Capabilities

None: the fixes bring nishiki to the `shiki-fidelity` spec as written.

## Impact

- `src/markdown/`, `src/vue.rs`, `src/csharp/`, `src/php/`, `src/ruby/` and `src/svelte/`.
- `src/latex/`, `src/bat/`, `src/python/`, `src/yaml/` and `src/blade/`.
- `src/json/`, `src/ini/`, `src/rust/`, `src/scss/`, `src/xml/`, `src/html/` and `src/diff/`.
- The `Language` docs in `src/language.rs`.

[gaps]: ../shiki-exact-gaps/tasks.md
