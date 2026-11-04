---
nishiki: patch
---

The TextMate engine runs a smaller program on a line with no character that a multi-character case fold needs, such as `ß` or `ﬁ`.
PowerShell, CMake, Batch and the other engine languages with case-insensitive patterns lex faster.
They also keep fewer automaton caches in memory.
