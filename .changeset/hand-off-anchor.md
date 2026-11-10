---
"nishiki": patch
---

A grammar's `while` rule can hand each line to a hand lexer, as a Markdown fence does.
The lexer now sees `\G` at the start of such a line, as in Shiki.
So in a shell fence, the line after a `\` reads as a new command.
