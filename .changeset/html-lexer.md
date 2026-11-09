---
nishiki: patch
---

Highlight HTML with the scopes of Shiki's `html` grammar.
The CSS of `<style>` elements goes to the CSS lexer.
The JavaScript of `<script>` elements and event handlers goes to the JavaScript lexer.
Markdown fences named `html`, `htm`, `shtml`, `xhtml`, `inc`, `tmpl` or `tpl` are highlighted as HTML; they were plain text before.
