---
nishiki: patch
---

The TextMate engine allocates the cells of its compiled patterns once per process, on first use.
It no longer ships them as static data.
A build with every engine language is about 400 KB smaller.
