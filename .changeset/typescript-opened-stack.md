---
nishiki: patch
---

Lex TypeScript and JavaScript faster: a line keeps only the contexts it opens, instead of filling two tables of every nesting depth, and a deeply nested document no longer allocates on each line.
