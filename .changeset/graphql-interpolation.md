---
nishiki: patch
---

Lex a GraphQL `${...}` interpolation with the JavaScript lexer across lines, as Shiki does.
It runs up to the `}` that no context of the code is waiting for, instead of counting braces on one line.
