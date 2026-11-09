---
nishiki: patch
---

Read the Unicode spaces that Shiki's `\s` reads as blanks in TypeScript, JavaScript, JSX and TSX.
They are U+0085, U+00A0, U+1680, U+2000 to U+200A, U+2028, U+2029, U+202F, U+205F and U+3000.
A keyword or a literal next to one is then highlighted as it is in Shiki.
