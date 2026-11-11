---
nishiki: patch
---

The GraphQL lexer allocates nothing per line: it reuses the buffer that holds the open rules from the line before.
