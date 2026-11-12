---
"nishiki": patch
---

Vue's `\s` matches Oniguruma's Unicode white space, such as U+00A0 and U+3000, as Shiki's does.
It applies in a block's end tag, around the `=` of a block's `lang`, at the end of a directive's bare value and between `<!--` and `@`.
