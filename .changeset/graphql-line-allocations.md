---
nishiki: patch
---

The GraphQL lexer allocates nothing per line: it reuses the buffer of open rules from the line before.
