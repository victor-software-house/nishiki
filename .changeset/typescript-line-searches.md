---
nishiki: patch
---

Search the rest of a line for a backtick, a `(` or a `<` once per line in the TypeScript lexer, not once per text an embedding language lexes with it.
A line of many HTML event handlers no longer reads the line up to each handler, and the work counted is the bytes the searches read.
