---
nishiki: patch
---

Count the Makefile lexer's open contexts above the document's own, as every other lexer does.
A pinned depth limit then opens as many contexts in Makefiles as elsewhere.
