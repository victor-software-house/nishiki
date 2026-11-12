---
"nishiki": patch
---

PHP and Blade close a heredoc or a nowdoc whose label follows Unicode white space, such as U+00A0 or U+3000, as Shiki does.
A parameter type that ends with U+00A0 gives back only that last space for the variable, so the type keeps Shiki's scopes.
