---
nishiki: patch
---

Add Vue.
The HTML lexer runs single-file components with the scopes of Shiki's `vue` grammar.
Each block's body runs on the lexer of the language its `lang` attribute names.
A Markdown `vue` fence holds Vue, as Shiki's `markdown-vue` grammar adds it.
