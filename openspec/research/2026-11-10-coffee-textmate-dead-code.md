# CoffeeScript with the TextMate engine: dead code

Which items does a build with `lang-coffee` and `textmate` leave unused, and why?

The set is the one `check:features` builds for CoffeeScript in a loaded Svelte grammar ([`mise-tasks/check/features.sh`][features], the `lang-coffee` and `textmate` test line).
That task allows dead code, so these items fail only a build that denies it.

## Answer

67 items are unused: 51 in the library and 16 more in its unit tests.
66 of them are the case the coding standards expect: shared code that the languages in the set do not call ([`CODING_STANDARDS.md`][standards], "check:features").
One is not.
`textmate::lex_handing_embedded` has an expectation whose condition misses this set (section 5).

An earlier run, at c8953c86, counted 45.
It counted distinct compiler messages, and one message can name several items.

## 1. Method

- The commit is 5c78f774, the head of [#709][pr709].
- Each set ran `cargo clippy --locked --lib --tests --no-default-features --features <set> -- -D warnings`.
  The sets are `lang-coffee textmate`, `lang-coffee` and `lang-javascript`.
  CoffeeScript turns on `lang-javascript` (`Cargo.toml`, `lang-coffee`).
- The JSON messages of each run list every dead item ([coffee-textmate], [coffee], [javascript]).
- A run with `--lib` alone gives the library's 51 ([coffee-textmate-lib]).
  An item in the full list but not in that one is dead only in the unit tests.

## 2. Counts

| Set | Library | Library and unit tests |
|:--|--:|--:|
| `lang-coffee textmate` | 51 | 67 |
| `lang-coffee` | not run | 80 |
| `lang-javascript` | not run | 84 |

## 3. Groups

| Group | Items | Also dead under `lang-javascript` | Where |
|:--|--:|:--|:--|
| Unicode classes and UTF-8 helpers | 17 | yes | library |
| Shared lexer helpers | 4 | yes | library |
| TypeScript lexer pieces | 3 | yes | library |
| Rule engine | 28 | no | 27 library, 1 unit tests |
| Test helpers | 14 | yes | unit tests |
| `textmate::lex_handing_embedded` | 1 | no | unit tests |

### 3.1 Unicode classes and UTF-8 helpers

- `src/classes.rs`: `is_upper`, `is_letter`, `is_number`, `is_mark`, `space_width`, `word_width` and `word_before`.
- `src/classes/table.rs`: `UPPERS`, `LETTERS`, `NUMBERS`, `MARKS`, and the four `is_*` functions that read them.
- `src/utf8.rs`: `char_start` and `char_at`.

`_classes` builds every class, and `lang-coffee` and `lang-javascript` both turn it on.
Neither lexer reads these classes.
Other lexers do: `rg` at 5c78f774 finds `word_before` in SQL, Ruby and C, and `char_at` in C, Kotlin and PHP.

### 3.2 Shared lexer helpers

- `Arena::path` (`src/arena.rs:195`);
- `Language::lex_from_start` and `past_end` (`src/language.rs:2363`, `2395`);
- `Limits::step` (`src/limits.rs:145`).

Each carries `expect(dead_code)` only under `not(feature = "_hand-lexers")`.
Other hand lexers call them: `lex_from_start` serves JSON, YAML, Markdown, HCL, TOML and Rust.

### 3.3 TypeScript lexer pieces

- `Dialect::TYPESCRIPT` and `Dialect::JSX` (`src/typescript.rs:97`, `101`);
- `start_script` (`src/typescript.rs:178`).

JavaScript runs the TypeScript lexer in its JavaScript dialect.
The other dialects' indices go unread.
`start_script` is called by the HTML engine, which this set does not build.

### 3.4 Rule engine

- `src/rules.rs`: the entries `lex`, `lex_until`, `lex_embedding_until_resuming`, `lex_on_demand` and `lex_again`.
- `src/rules.rs`: the variants `Visit`, `Backref`, `Embed`, `Lowered` and `Capture`.
- `src/rules/chars.rs`: `prev_is`, `keyword_out_of_line` and `word_in`.
- `src/rules/lexer.rs`: `run`, `run_halting`, `run_on_demand`, `at_g`, `capture_span`, `span`, `span_retok`, `span_dyn`, `span_one`, `clear_spans`, `payload_at`, `payload_text`, `payload_before`, `skip_plain` and `path_fixed`.
  `capture_span` is dead only in the unit tests.

CoffeeScript is the only rule-engine language in the set.
It enters the engine through `start`, `start_embedded`, `lex_embedding` and `lex_embedding_until` (`src/coffee.rs:47` to `88`).
The other entries, and the payloads its grammar never builds, go unused.
`lang-javascript` builds no rule engine, so none of these exist there.

### 3.5 Test helpers

- `src/test_support.rs`: `without_root`, `scopes_in`, `scopes_of`, `leaf`, `leaf_row`, `LexUntil`, `Embedded`, `RUBY_HALTS`, `ruby_interpolation`, `ruby_escape`, `lex_embedded`, `lex_embedded_stops` and `ruby_heredoc_halts`.
- `Highlighter::frame` (`src/highlighter.rs:272`).

Both build with tests and any hand lexer (`src/lib.rs:538` to `540`).
They serve other languages' tests, such as Ruby's halts.

## 4. What `textmate` changes

Against `lang-coffee` alone, `textmate` adds one dead item and makes 14 live.

- Added: `textmate::lex_handing_embedded`.
- Made live:
  - `coffee::grammar`'s `ALL`, `SvelteProps` and `SvelteVariable`;
  - `typescript::scope`'s `PunctuationDefinitionVariableSvelte`;
  - `Language::halt_before`, `Limits::into_spent`, `rules::backref` and `utf8::char_len`;
  - `rules::lexer`'s `Probe` and `probe_matcher`;
  - `test_support::probe`'s `After`, `Anchor`, `Case` and `lexer`.

The Svelte items are the loaded-grammar `$` prefixes of [#694][pr694].

## 5. The one expectation that misses

`textmate::lex_handing_embedded` expects dead code under `any(not(test), not(feature = "_hand-lexers"))` (`src/textmate.rs:165` to `172`).
Its one caller is a test in `mod hand_offs` (`src/textmate/tests.rs:2296`).
That module builds only under `all(feature = "lang-haml", feature = "lang-objective-c")` (`src/textmate/tests.rs:1959`).

With tests and a hand lexer on, but not both HAML and Objective-C, the function is dead and nothing expects it.
`lang-coffee textmate` is one such set.
`check:features` allows dead code, so no check reports it.
An expectation under `not(all(test, feature = "lang-haml", feature = "lang-objective-c"))` would match the caller's gate.

## Not verified

- The causes in sections 3.1 to 3.3 and 3.5 come from reading the code and from `rg`, not from a run per caller.
- The unit tests reported 41 errors.
  The text log lists only the 16 beyond the library's 34, so the library items the unit tests share were not listed one by one.
- The library counts of `lang-coffee` and `lang-javascript` alone were not run.
- No other feature set was run.
- The condition in section 5 was not built.

## Compute

- Two rounds on an ephemeral build box, 4 vCPU and 8 GiB, on the build host.
- Each ended within its one-hour limit; the wall time was not recorded.
- No billed cost.

[features]: ../../mise-tasks/check/features.sh
[standards]: ../../CODING_STANDARDS.md
[pr709]: https://github.com/victor-software-house/nishiki-archive/pull/709
[pr694]: https://github.com/victor-software-house/nishiki-archive/pull/694
[coffee-textmate]: 2026-11-10-coffee-textmate-dead-code/coffee-textmate.tsv
[coffee]: 2026-11-10-coffee-textmate-dead-code/coffee.tsv
[javascript]: 2026-11-10-coffee-textmate-dead-code/javascript.tsv
[coffee-textmate-lib]: 2026-11-10-coffee-textmate-dead-code/coffee-textmate-lib.tsv
