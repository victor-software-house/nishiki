---
nishiki: patch
---

The TypeScript lexer searches the rest of a line for a backtick, a `(` or a `<` once per line.
It no longer searches once per text that an embedding language lexes with it.
A line of many HTML event handlers no longer reads the line up to each handler.
The work counted is the bytes the searches read.
