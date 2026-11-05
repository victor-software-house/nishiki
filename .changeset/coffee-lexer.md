---
nishiki: patch
---

Highlight CoffeeScript with a lexer of its own, on by default and in `all`.
The `lang-coffee` feature leaves the opt-in `textmate` tier.
A default build now highlights Markdown's `coffee` fences and the `<script lang="coffee">` of Vue and Svelte, which it left plain before.
HAML's `:coffee` filter uses the same lexer.
