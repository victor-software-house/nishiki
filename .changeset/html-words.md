---
nishiki: patch
---

HTML, its derivatives, ERB and PostCSS compare tag names and literals in byte loops instead of libc's `memcmp`.
