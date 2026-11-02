---
nishiki: patch
---

Build only the languages you highlight: each lexer is a `lang-*` feature, `default` turns them all on, and the tiers `config`, `web`, `systems` and `docs` turn on groups. `Language::ALL` and `Language::from_name` know the languages that are built, and `json` stays the theme-file feature.

A dependency written `default-features = false, features = ["json"]` built every language in 0.1.0 and now builds none. Add the `lang-*` features it uses, or `all`.
