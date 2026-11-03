---
nishiki: patch
---

Count the Makefile lexer's open contexts above the document's own, as every other lexer does, so a pinned depth limit opens as many in Makefiles as elsewhere.
