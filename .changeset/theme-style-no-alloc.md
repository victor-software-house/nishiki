---
nishiki: patch
---

`Theme::style` allocates nothing for stacks of up to 64 scopes.
Colouring a document runs about 7% fewer instructions for the median language, and JSON about 31% fewer.
