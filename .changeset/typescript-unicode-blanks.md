---
nishiki: patch
---

Read the Unicode spaces that Shiki's `\s` reads (U+0085, U+00A0, U+1680, U+2000 to U+200A, U+2028, U+2029, U+202F, U+205F and U+3000) as blanks in TypeScript, JavaScript, JSX and TSX, so a keyword or a literal next to one is highlighted as it is in Shiki.
