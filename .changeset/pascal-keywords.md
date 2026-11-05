---
nishiki: patch
---

Pascal and Common Lisp look their keywords up in a byte loop.
Pascal runs about 8% fewer instructions, and Common Lisp about 11% fewer.
Their counts no longer follow where the words lie.
