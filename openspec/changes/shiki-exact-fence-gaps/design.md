# Design

## 1. Measure

A fix is measured as [`shiki-exact-gaps`' design][design] section 1 says: on a box, at seeds 1, 2 and 3.
Each task first reproduces its case against Shiki in a probe, then in a test.

## 2. Sources

Each case comes from a run that [`shiki-exact-gaps`][tasks] task 6.4 made:

- a sweep that puts a `vue` fence inside a fence of each language with oracle fixtures, 73 in all;
- the `win` and `mdw` corpora of Markdown, Svelte, Haml, TOML and Vue;
- seed 2's `mdw` corpora of the 73 languages, and Svelte's differential.

The cases outside Vue fences differ at the base too.

[design]: ../shiki-exact-gaps/design.md
[tasks]: ../shiki-exact-gaps/tasks.md
