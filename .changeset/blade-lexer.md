---
nishiki: patch
---

Highlight Blade, Laravel's templates, behind the `lang-blade` feature, on by default and in the `all` and `web` tiers.
The HTML lexer runs its tags, echoes and directives, and a copy of PHP's rules lexes the PHP inside them.
An echo or a directive inside that PHP opens at any depth, as in Shiki, such as `{{ "a {{ b }} c" }}`.
