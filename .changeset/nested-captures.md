---
nishiki: patch
---

The TextMate engine stops nesting capture groups at 256 on a line, and reports the depth limit there.
A line that nested more, such as two thousand Perl heredoc openers, overflowed the stack of a debug build.
