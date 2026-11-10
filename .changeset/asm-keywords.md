---
nishiki: patch
---

Assembly looks its keywords up without libc's `memcmp`.
Its instruction count stays the same wherever the words lie in the binary.
