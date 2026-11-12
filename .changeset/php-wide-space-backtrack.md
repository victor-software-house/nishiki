---
"nishiki": patch
---

PHP and Blade give back each Unicode space before a name until the name can start, as Shiki does.
`A::`, U+00A0 and a space before `()` make the U+00A0 the method's name.
