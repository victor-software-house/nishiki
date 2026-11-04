---
nishiki: patch
---

Lex deeply nested lines faster on the rule engine.
A line no longer visits every open rule to end.
A Ruby document 300 levels deep takes a quarter fewer instructions.
