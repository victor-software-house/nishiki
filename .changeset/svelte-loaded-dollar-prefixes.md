---
"nishiki": patch
---

A Svelte grammar loaded through `Grammar::from_json` now gives the JavaScript and TypeScript it hands off Svelte's `$` prefixes.
The `$` of `$count` is `punctuation.definition.variable.svelte` there, as in Shiki and in nishiki's own Svelte.
This holds with or without the `lang-svelte` feature.
HTML in such a grammar gives its `<script>` the prefixes too.

Before, the handed code read `$` as part of the name.
