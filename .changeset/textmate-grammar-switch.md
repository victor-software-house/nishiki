---
nishiki: patch
---

Keep the TextMate engine's caches for each grammar when a document switches between grammars, as a Markdown document with fences in two of the engine's languages does, so lexing it again allocates nothing.
