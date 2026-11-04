---
"nishiki": patch
---

The languages on the rule engine, and shell, run up to 0.6% fewer instructions.
Their stack cache no longer moves when another language's tables change.
A highlighter allocates 16 KiB less, because shell's cache is made only when shell lexes a line.
