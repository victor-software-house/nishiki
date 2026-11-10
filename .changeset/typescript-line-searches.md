---
nishiki: patch
---

The TypeScript lexer searches the rest of a line for a backtick, a `(` or a `<` once per line.
It does not search once per text that an embedding language lexes with it.
A line of many HTML event handlers does not read the line up to each handler.
The work counted is the bytes the searches read.
