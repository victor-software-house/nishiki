---
nishiki: patch
---

Lex lines inside deeply nested rules faster with the TextMate engine.
Each line reuses the open rules the previous line left.
It touches only the rules it opens or closes and the open `while` rules.
