---
nishiki: patch
---

Pascal and Common Lisp look their keywords up in a byte loop.
Pascal runs about 8% fewer instructions, and Common Lisp about 11% fewer.
Their counts stay the same wherever the words lie.
