---
nishiki: patch
---

The GraphQL lexer no longer allocates per line: it reuses the buffer of open rules from the line before.
