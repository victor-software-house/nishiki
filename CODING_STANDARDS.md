# Coding standards

Numbered rules for nishiki's code.
Reviews cite the number.
`AGENTS.md` holds the invariants these rules serve; the lints in `Cargo.toml` and `clippy.toml` enforce what a machine can check, each with its reason.

## 1. Lexers

### 1.1 One contract per language

A language is a module `src/<language>.rs` beside `src/<language>/`, with three crate-private functions: `start(arena, base) -> u32`, the state at the start of a document whose scopes sit on `base`; `start_embedded(arena, base) -> u32`, the same inside a Markdown fence, with no root scope of its own; and `lex(cx, line, state) -> u32`, which lexes one line from a state, appends its tokens, records the limits it reached, and returns the next state.
`lex` takes a `Context` (`src/language.rs`: the shared arena, the limits, the crate-level `Scratch`, the token buffer and the report) and a `Line` (the whole line, the byte where the language's text starts, and whether a fence embeds it), so an embedding language can pass a fence line whole and `^` still matches only at column 0; token ends count from the start of the whole line, and `Context::lex_from_start` moves them there for a lexer that counts from its own start.
`Scratch` (`src/scratch.rs`) holds one field per language: buffers and caches the `Highlighter` owns and keeps across lines, cleared rather than reallocated; a language uses its own field and those of the languages it embeds.
`src/language.rs` is the one place they are called: `Language::start`, `Language::start_embedded` and `Language::lex` each have one arm per language that calls the module's function.
The `Highlighter` and the Markdown fences both call `Language::lex`, so a new language is wired in by appending its arms there, its scope table, its frame tag and its fence names; a language has no highlighter, interner or state type of its own.
One contract keeps every language reachable through one public API, and lets one language embed another on the same arena.

### 1.2 State lives in the arena

A lexer's open contexts are frames of the shared arena (`src/arena.rs`), pushed and popped through the lexer's `push`, `push_with`, `pop`, `set_payload` and `payload`.
A frame's kind is the language's context enum, tagged with the language's own high byte.
Every tag is a constant in `src/tag.rs`, with a compile-time check that no two ranges overlap, including a lexer that claims a wider range for flag bits; a lexer takes its constant there and never the next free byte, because lexers are written in parallel.
Data a context carries, such as a raw string's `#` count, a fence's delimiter or an embedded language's state, is the frame's payload; text is interned first, so the payload is an id.
Equal states then have equal ids, which is what lets an editor stop re-lexing as soon as a line's state matches the old one.

### 1.3 Scope names are tables

Each language lists its scope names once, as an enum whose discriminant indexes a `NAMES` table, in Shiki's spelling.
`src/scope.rs` numbers every table in one space, and each table's base is computed from the previous table's end (`after`), never written by hand, so adding a language cannot make two tables overlap; a compile-time check fails the build when a table does not start where the one before it ends, as a merge that keeps both sides can leave it.
A name the grammar misspells keeps the grammar's spelling, written with one byte escape in the literal so the stock spelling check passes.

### 1.4 Every limit comes from `Limits`

No lexer has a hidden limit that changes output.
Nesting depth, work, line length and interned nodes come from the caller's `Limits`; a recursion guard that cannot be configured is a named constant in the language's `limits.rs`, with its reason, and the README lists it.
Every scan that reads ahead of the cursor counts its bytes with `spend`, and the main loop checks the budget each turn.
When a limit is reached the line still gets tokens that cover it, the report names the limit, and the state stays usable.

### 1.5 Every step makes progress

The main loop asserts progress in debug builds.
Where a grammar's rules match zero-width and continue at the same byte, the assertion bounds the run of steps that do not advance, and the bound is written down beside it.
The fuzz test and the work limit catch what the assertion misses.

### 1.6 Embedding

A language embedded in another (Rust in a Markdown fence) runs on the embedding lexer's arena, through a `start_embedded` that starts on the fence's stack without a root scope of its own, as Shiki embeds it.
Its state is the payload of the fence's frame, and it shares the embedding line's work and depth limits.
A fence's kind holds the embedded `Language`, and Markdown numbers its kinds from `Language::ALL`, so adding a fence language is one entry in the fence table and its embedded scope in `rules::content_name`.

## 2. Speed

### 2.1 The hot path allocates nothing

Lexing a line allocates nothing per token and nothing per line once the arena holds the stacks the document uses.
Buffers that live across lines belong to the highlighter; a lexer reuses them rather than building them anew each line.

### 2.2 Scan, do not test byte by byte

Find the next interesting byte with `memchr` and its relatives, dispatch on a line's first byte or a context's first-byte table rather than trying every rule in order, and compute a line-wide lookahead once per line (as the Rust lexer finds the line's last `>(` once).

### 2.3 Measure before claiming

A change that claims to be faster carries its `mise run bench` table before and after, from one machine (see `bench/AGENTS.md`).
A speed change never changes output: the oracle, colour, fuzz and limit tests stay green.

## 3. Code

### 3.1 Module layout

A module with submodules lives in `foo.rs` beside `foo/`, never `mod.rs`.
A file is named for the one concern it owns (`strings.rs`, `heredoc.rs`); names such as `misc.rs`, `utils.rs` or `decl2.rs` say nothing and are refused in review.
An item is `pub(crate)` unless the public API needs it.

### 3.2 No panics in library code

Library code returns typed values or errors and never panics: an invariant is an exhaustive `match` with no `_` arm, or a typed error.
Tests may unwrap.

### 3.3 Exceptions are local and explained

A lint exception is `#[expect(lint, reason = "…")]` on the smallest item; `#[allow]` is denied.
`as` is denied except on a site whose `#[expect]` says why the conversion cannot lose data.

### 3.4 Comments carry the why

Doc comments say why, on the item; inline prose inside a body is removed in review.
Multiline text in code and tests uses `indoc!` or `formatdoc!`.

### 3.5 Dependencies

The crate depends on `memchr` alone; a feature may add one dependency when it removes more work or risk than it adds, and the change says why (`json` adds `serde_json`).
A dependency is pure Rust; one that builds C or carries unsafe code needs a recorded reason.

### 3.6 Public API

The API follows the Rust API guidelines: `new` and `new_with_*` constructors, getters by value only for `Copy` types, enums rather than `bool` parameters, `#[non_exhaustive]` where variants will grow.
A change that breaks it is called out in its changeset; the release rules in `AGENTS.md` decide its version.

## 4. Tests

### 4.1 Shiki is the test

A language is done when every oracle fixture matches Shiki on every non-whitespace byte, in scopes and in colours under every oracle theme, held-out inputs included.
Unit tests cover what the oracle cannot reach: states across lines, limits, embedding, edge cases a fixture lacks.

### 4.2 The bar for a new language

Before its pull request opens, a language has: development and held-out oracle inputs with their licences in `tests/oracle/SOURCES.md`, its grammar's notice in `THIRD-PARTY-NOTICES.md`, the colour test, its hostile lines in `tests/it/limits.rs`, a pass of the fuzz test, and its `mise run bench` table.

### 4.3 One integration binary

Integration tests are modules of `tests/it/main.rs`; a new file is a module there, not a new binary.

## 5. Crates and publishing

### 5.1 One published crate until a real boundary

nishiki is one published crate.
A part becomes its own crate only at a real boundary, shown by how often it changes and who depends on it, or by a build-time measurement: likely the TextMate engine's bundled grammar data, whose size would rebuild everything else.
Crates are named `nishiki-<noun>`, never with `-rs`.

### 5.2 Languages are features

When the language list grows past what every user wants, each language becomes a cargo feature, as tree-shaking tiers, and `check:features` proves each advertised set builds alone.

### 5.3 What ships

`include` in `Cargo.toml` lists what a published crate ships; tests, fixtures, the oracle, benchmarks and docs never ship.
A workspace member that is not for crates.io sets `publish = false`, and `.ctl/ver.yaml` lists only the packages that publish.
Measurement and research programs live in their own workspace, excluded from the root, so the root's lints, deny rules and versions never read them.
