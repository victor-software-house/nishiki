---
nishiki: patch
---

Read the Unicode spaces that Shiki's `\s` reads (U+0085, U+00A0, U+1680, U+2000 to U+200A, U+2028, U+2029, U+202F, U+205F and U+3000) as blanks in Rust, and not as word characters, so `fn` and `pub` before one are keywords as they are in Shiki.
