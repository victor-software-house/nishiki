---
"nishiki": patch
---

A Svelte grammar loaded through `Grammar::from_json` now gives the CoffeeScript it hands off Svelte's `$` prefixes too.
The `$` of `$count` is `punctuation.definition.variable.svelte` there, as in Shiki.
This holds with or without the `lang-svelte` feature.

Before, the handed CoffeeScript read `$` as part of the name.
