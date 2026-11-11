---
nishiki: patch
---

Colouring the first line of C costs about 2 million instructions instead of 250 million.
Each context's pattern tables are built the first time a line reaches it, not all at once.
