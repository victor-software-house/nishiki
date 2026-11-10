---
"nishiki": patch
---

A grammar loaded through `Grammar::from_json` that hands a region to a lexer now gives it `\G` where Shiki does.
After a begin match that took the line feed, `\G` matches at the start of each later line while the lexer has only its root open.
A Makefile line that starts with a tab is then an assignment there, as in Shiki.

Before, `\G` never matched at the start of those lines.
