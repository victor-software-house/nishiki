---
nishiki: patch
---

`Limits::depth` leaves out a language's root context in JSON, TOML, YAML, Dockerfile and INI, as in the other languages.
Under a pinned depth, these open one more nested context than before.
