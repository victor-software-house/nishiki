---
nishiki: patch
---

`Limits::depth` no longer counts a language's root context, in Rust and Markdown as in the languages lexed by the shared rule engine: under a pinned depth, Rust and Markdown open one more nested context than before.
