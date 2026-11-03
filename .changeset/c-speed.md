---
nishiki: patch
---

Make the C lexer 6 to 9 percent faster by keeping its buffers from line to line instead of allocating them for each line.
