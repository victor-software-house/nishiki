---
nishiki: patch
---

The Rust lexer no longer stops advancing on an uppercase-led word followed by non-ASCII text, such as `R日`, which made it spin until the work limit.
