---
nishiki: patch
---

Languages on the shared rule engine no longer try a pattern where its leading lookbehind rules it out.
Such places are the start of the line and the inside of a word.
Astro runs about 5% fewer instructions, Scala and Go about 4%, and Swift about 3%.
