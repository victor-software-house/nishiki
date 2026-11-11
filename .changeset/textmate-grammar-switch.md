---
nishiki: patch
---

Keep the TextMate engine's caches for each grammar when a document switches between grammars.
A Markdown document that has fences in two of the engine's languages switches that way.
Lexing such a document again allocates nothing.
