---
"nishiki": patch
---

PHP and Blade give a name the last Unicode space before it, such as U+00A0 or U+3000, when no name starts after the spaces, as Shiki does.
A line of spaces and `:` makes the last space a goto label, and `::` with U+00A0 before `(` makes it a method name.
