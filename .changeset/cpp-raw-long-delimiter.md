---
nishiki: patch
---

Close a C++ raw string whose delimiter is longer than 16 bytes on that delimiter, as Shiki does, instead of on `)"` alone.
