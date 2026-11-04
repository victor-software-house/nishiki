---
nishiki: patch
---

The first token of a language on nishiki's TextMate engine comes sooner.
A rule list's scanner now extracts its prefilter with a tighter limit than `regex-automata`'s.
A list of literals keeps `regex-automata`'s own extraction.
