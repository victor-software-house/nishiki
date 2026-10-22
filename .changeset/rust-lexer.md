---
nishiki: patch
---

Highlight Rust: `nishiki::rust::Highlighter` lexes one line at a time and gives each token the scope stack that Shiki's Rust grammar gives it, so a TextMate theme colours it as Shiki does.
