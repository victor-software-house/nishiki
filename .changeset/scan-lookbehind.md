---
nishiki: patch
---

Languages on the shared rule engine no longer try a pattern where its leading lookbehind rules it out: at the start of the line, or inside a word.
Astro runs about 5% fewer instructions, Scala and Go about 4%, and Swift about 3%.
