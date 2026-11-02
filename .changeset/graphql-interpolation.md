---
nishiki: patch
---

Lex a GraphQL `${...}` interpolation with the JavaScript lexer across lines, up to the `}` that no context of the code is waiting for, as Shiki does, instead of counting braces on one line.
