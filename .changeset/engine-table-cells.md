---
nishiki: patch
---

The TextMate engine allocates the cells of its compiled patterns once per process, on first use, instead of shipping them as static data, so a build with every engine language is about 400 KB smaller.
