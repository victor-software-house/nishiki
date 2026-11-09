---
nishiki: patch
---

Lex TypeScript and JavaScript faster.
A line keeps only the contexts it opens, instead of filling two tables of every nesting depth.
A deeply nested document no longer allocates on each line.
