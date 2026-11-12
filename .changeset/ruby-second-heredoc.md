---
"nishiki": patch
---

After a Ruby here document opener, a second opener on the same line takes the rest of the line into its embedded language, as Shiki does.
That text gets the embedded language's scopes.
Before, it kept Ruby's scopes.
