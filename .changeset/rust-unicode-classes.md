---
"nishiki": patch
---

Rust's `\w`, `\b` and `\d` match Unicode letters and digits, as Shiki does.
An Arabic-Indic digit such as `٣` starts a number, and an emoji such as `🦀` ends a name.
