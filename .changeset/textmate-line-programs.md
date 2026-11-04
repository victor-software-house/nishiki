---
nishiki: patch
---

The TextMate engine runs a lean program on each line that holds no character a multi-character case fold needs, such as `ß` or `ﬁ`, so PowerShell, CMake, Batch and the other engine languages with case-insensitive patterns lex faster and keep fewer automaton caches in memory.
