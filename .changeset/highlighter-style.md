---
nishiki: patch
---

Add `Highlighter::style`, which gives a token's style from its stack.
It gives the style `Theme::style` gives, and keeps each stack's style once computed.
It keeps the styles of the two themes used last, so styling each token with a light and a dark theme stays cached.
Colouring through it runs about 53% fewer instructions than through `Theme::style`, and JSON about 96% fewer.
