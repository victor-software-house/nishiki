---
"nishiki": patch
---

A grammar can hand a region to a hand lexer and keep rules of its own beside the include.
Those rules now apply only where the lexer has no context of its own open, as in Shiki.

Before, such a rule that matched at the start of a line applied inside the lexer's open contexts too.
In HTML, a `//` comment line inside a JavaScript object then lost the object's scopes.
