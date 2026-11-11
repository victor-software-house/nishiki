---
nishiki: patch
---

Highlight HAML with a lexer of its own, enabled by default and in `all`.
The `lang-haml` feature leaves the opt-in `textmate` tier.
It turns on Ruby, JavaScript, CSS, Sass, CoffeeScript and Markdown, which HAML's filters and Ruby lines embed.
Ruby's `<<~HAML` here documents are now highlighted as HAML in a default build, where they were plain text before.
