---
nishiki: patch
---

Highlight Svelte with a lexer of its own, behind the `lang-svelte` feature.
The feature is in the default `all` tier and in `web`.

- Its expressions and `<script>` bodies go to the TypeScript and JavaScript lexers.
- Its `<style>` bodies go to the CSS, PostCSS, SCSS, Less and Sass lexers.
- A `lang="coffee"` body goes to the CoffeeScript lexer when `lang-coffee` is on.
- An `@component` comment goes to the Markdown lexer when `lang-markdown` is on.
