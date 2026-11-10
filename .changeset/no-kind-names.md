---
nishiki: patch
---

Rename the two public types whose names held "kind", and the field and methods that use them.

This breaks every caller that names them:

- `ThemeKind` is now `ColorScheme`.
  `Theme::kind` is now `Theme::color_scheme`, and `ThemeSource`'s `kind` field is now `color_scheme`.
- `LineKind` is now `Freshness`, and `DocumentLine::kind` is now `DocumentLine::freshness`.

Each variant keeps its name, and no output changes.
