---
nishiki: patch
---

Highlighting a line pays no call into the language dispatch for diff, Dockerfile, YAML, TOML, JSON, INI, GraphQL and Markdown.
In a default release build, those languages run 0.3% to 4.8% fewer instructions.
