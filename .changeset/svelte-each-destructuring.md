---
"nishiki": patch
---

Svelte lexes the line after an open `{#each}` tag as Shiki does.
Where that line starts with `{…}` and more text follows, the text keeps the destructuring's scopes.
Before, the line reached the work limit.
