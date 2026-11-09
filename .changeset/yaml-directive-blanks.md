---
nishiki: patch
---

YAML directives read only spaces and tabs between their words, as Shiki does.
A word after a no-break space is marked as a mistake, and so is a reserved directive's third word.
The spaces before a directive's comment or line end no longer take the directive's scope.
