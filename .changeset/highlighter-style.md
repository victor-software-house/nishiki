---
nishiki: patch
---

Add `Highlighter::style`, which gives a token's style from its stack.
It gives the style `Theme::style` gives, and keeps each stack's style once computed.
Colouring through it runs about 53% fewer instructions than through `Theme::style`, and JSON about 96% fewer.
