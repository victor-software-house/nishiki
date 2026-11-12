---
"nishiki": patch
---

A Mermaid click's tooltip string ends at the line's last `"`, as in Shiki.
Text after it, such as a trailing backslash, keeps its own scopes, and a click line with no `"` is no click.
