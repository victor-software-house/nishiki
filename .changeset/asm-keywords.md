---
nishiki: patch
---

Assembly looks its keywords up without libc's `memcmp`.
Its instruction count no longer follows where the words lie in the binary.
