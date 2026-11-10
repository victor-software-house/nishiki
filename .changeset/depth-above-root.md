---
nishiki: patch
---

`Limits::depth` leaves out a language's root context in Rust and Markdown, as in the languages lexed by the shared rule engine.
Under a pinned depth, Rust and Markdown open one more nested context than before.
