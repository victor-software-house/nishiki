---
"nishiki": patch
---

A loaded grammar can hand a region to a language when the region's end or the rules beside the include use `\G`.
Before, the include matched nothing there, or the link failed when the language was named with `Grammars::hand_off`.
