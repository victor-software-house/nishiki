# Coding standards

Numbered rules for nishiki's code.
Reviews cite the number.

- `AGENTS.md` holds the invariants these rules serve.
- The lints in `Cargo.toml` and `clippy.toml` enforce what a machine can check, each with its reason.
- This file states only what nishiki adds or decides.
  The general rules come from:
  - the `rust-projects` skill's code rules;
  - the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/);
  - [Google's Rust style guide](https://google.github.io/styleguide/rustguide.html), its general rules, not the Chrome, Fuchsia or Android ones;
  - [BeautifulRust's rust-best-practices](https://github.com/morganlinton/BeautifulRust).

## 1. Lexers

### 1.1 One contract per language

A language is a module `src/<language>.rs` beside `src/<language>/`,
with three crate-private functions:

| Function | Returns |
|:--|:--|
| `start(arena, base) -> u32` | The state at the start of a document whose scopes sit on `base`. |
| `start_embedded(arena, base) -> u32` | The same inside a Markdown fence, with no root scope of its own. |
| `lex(cx, line, state) -> u32` | The next state, after it lexes one line, appends its tokens and records the limits it reached. |

`lex` takes two values, both defined in `src/language.rs`:

- a `Context`: the shared arena, the limits, the crate-level `Scratch`, the token buffer and the report;
- a `Line`: the whole line, the byte where the language's text starts, and whether a fence embeds it.

An embedding language can therefore pass a fence line whole, and `^` still matches only at column 0.

- Token ends count from the start of the whole line.
- `Context::lex_from_start` moves them there, for a lexer that counts from its own start.
- `Scratch` (`src/scratch.rs`) holds one field per language:
  buffers and caches the `Highlighter` owns and keeps across lines, cleared rather than reallocated.
  A language uses its own field and those of the languages it embeds.

`src/language.rs` holds one `languages!` table, a row per language.
The table generates `Language`, `Language::ALL`, `Language::id`, the names `Language::from_name` reads, and `Language::start`, `Language::start_embedded` and `Language::lex`, each with one arm per language that calls the module's function.
The `Highlighter` and the Markdown fences both call `Language::lex`.
A row names the language's module, and a dialect when one module serves several languages.

A new language is wired in with:

1. its row in `src/language.rs`;
2. its scope table: one line in `src/scope.rs`;
3. its frame tag in `src/tag.rs`;
4. its fence names in `src/markdown/fence.rs`;
5. its `lang-*` feature in `Cargo.toml`, which turns on `_language`, with a `cfg` on each of the above (5.2).

The scope tables, the frame tags and the `lang-*` features are sorted by language, and so are the other lists a language adds to: the README's language table, `src/scratch.rs`, `mise-tasks/check/features.sh`, `oracle/inputs.ts`, `tests/oracle/SOURCES.md`, Markdown's fence languages and fence scopes in `src/markdown/rules.rs` and `src/markdown/scope.rs`, and the sections of `THIRD-PARTY-NOTICES.md`.
A new row goes in its sorted place, never at the end, so two languages added in parallel edit different lines.
`tests/it/sorted.rs` fails when a list is out of order.
The rows of `src/language.rs` stay in the order they were added, with a new row at the end, until the next minor release sorts them: their order sets the discriminants of the public `Language` enum and the order of `Language::ALL`, so moving one is an API break.

A language has no highlighter, interner or state type of its own.
One contract keeps every language reachable through one public API,
and lets one language embed another on the same arena.

### 1.2 State lives in the arena

- A lexer's open contexts are frames of the shared arena (`src/arena.rs`).
  They are pushed and popped through the lexer's `push`, `push_with`, `pop`, `set_payload` and `payload`.
- A frame's kind is the language's context enum, tagged with the language's own high byte.
- Every tag is a constant in `src/tag.rs`.
  A compile-time check fails when two ranges overlap,
  including a lexer that claims a wider range for flag bits.
- A lexer takes its constant there, never the next free byte, because lexers are written in parallel.
- Data a context carries is the frame's payload:
  a raw string's `#` count, a fence's delimiter, an embedded language's state.
  Text is interned first, so the payload is an id.

Equal states then have equal ids.
That is what lets an editor stop re-lexing as soon as a line's state matches the old one.

### 1.3 Scope names are tables

- Each language lists its scope names once, in Shiki's spelling,
  as an enum whose discriminant indexes a `NAMES` table.
- `src/scope.rs` numbers every table in one space.
  A table's slot is the high byte of its language's frame tag, and a scope number is the slot in the high 16 bits and the name's index in the low 16.
  One table's length therefore never moves another table's numbers, and no language needs a number of its own.
- A compile-time check fails the build when a table outgrows its slot.
- A language whose names are one list declares them with `scope_table!`, which generates the enum, `NAMES` and the conversion to a scope.
- A name the grammar misspells keeps the grammar's spelling.
  It is written with one byte escape in the literal, so the stock spelling check passes.

### 1.4 Every limit comes from `Limits`

No lexer has a hidden limit that changes output.

- Nesting depth, work, line length and interned nodes come from the caller's `Limits`.
- A recursion guard that cannot be configured is a named constant in the language's `limits.rs`, with its reason.
  Its `Language` variant's docs list it.
- Every scan that reads ahead of the cursor counts its bytes with `spend`.
  The main loop checks the budget each turn.
- When a limit is reached:
  - the line still gets tokens that cover it;
  - the report names the limit;
  - the state stays usable.

### 1.5 Every step makes progress

- The main loop asserts progress in debug builds.
- A grammar's rules may match zero-width and continue at the same byte.
  There the assertion bounds the run of steps that do not advance, and the bound is written down beside it.
- The fuzz test and the work limit catch what the assertion misses.

### 1.6 Embedding

A language embedded in another, such as Rust in a Markdown fence, runs on the embedding lexer's arena.

- It starts through `start_embedded`, on the fence's stack, without a root scope of its own, as Shiki embeds it.
- Its state is the payload of the fence's frame.
- It spends the embedding line's work budget.
  - Each lexer counts in a `Steps` field of its own, against the limit its `Limits` give it.
  - The embedding lexer passes it limits whose work is what the line has left (`Steps::lend`).
  - Every lexer's `lex` returns what it spent beside its state (`Finished`).
  - The embedding lexer adds that to its own count (`Steps::take_back`).
- It counts only its own contexts against the depth limit.
  A pinned depth then gives it the same output wherever it is embedded.
- A fence's kind holds the embedded `Language`, and Markdown numbers its kinds from `Language::ALL`.
  Adding a fence language is one entry in the fence table and its embedded scope in `rules::content_name`.

## 2. Speed

### 2.1 The hot path allocates nothing

- Lexing a line allocates nothing per token,
  and nothing per line once the arena holds the stacks the document uses.
- Buffers that live across lines belong to the highlighter.
  A lexer reuses them rather than building them anew each line.

### 2.2 Scan, do not test byte by byte

- Find the next interesting byte with `memchr` and its relatives.
- Dispatch on a line's first byte, or a context's first-byte table, rather than trying every rule in order.
- Compute a line-wide lookahead once per line, as the Rust lexer finds the line's last `>(` once.

### 2.3 Measure before claiming

- A change that claims to be faster carries its `mise run bench` table before and after, from one machine (see `bench/AGENTS.md`).
- A speed change never changes output: the oracle, colour, fuzz and limit tests stay green.

### 2.4 A forced trade-off means the shape is wrong

- When fidelity and speed cannot both hold, treat it as a sign that the design is wrong, not as a price to pay (operator, 2026-11-04).
  Rework the data layout or the point where the work happens until both hold.
- A cost accepted to land a change, such as a language over the Callgrind gate, is temporary.
  The same pull request names the follow-up that removes it, and that follow-up is planned work.

### 2.5 A lazy cell's closure captures by value

- On a hot path, the closure given to `get_or_init` is `move` and captures only `Copy` values, such as an index or a shared reference.
  A closure that borrows its captures builds their environment on the stack before the cell's initialized test, on every call, though it runs only once.

### 2.6 State on the hot path has no `Drop` impl

- A value a lexer holds while it lexes a line, such as its `Steps`, has no `Drop` impl.
  Drop glue turns every call made while the value lives into an unwinding call with a cleanup path.
  That path grows the caller and changes what LLVM inlines into it.
- Work that must happen when the value is done goes in one consuming method.
  The type system makes every exit call it.
  For example, `Steps::finish` is the only way to make the `Finished` that carries a lexer's state.

## 3. Code

### 3.1 Module layout

- A module with submodules lives in `foo.rs` beside `foo/`, never `mod.rs`.
- A file is named for the one concern it owns (`strings.rs`, `heredoc.rs`).
  Names such as `misc.rs`, `utils.rs` or `decl2.rs` say nothing and are refused in review.
- An item is `pub(crate)` unless the public API needs it.

### 3.2 No panics in library code

- Library code returns typed values or errors and never panics.
- An invariant is an exhaustive `match` with no `_` arm, or a typed error.
- Tests may unwrap.

### 3.3 Exceptions are local and explained

- A lint exception is `#[expect(lint, reason = "…")]` on the smallest item.
  `#[allow]` is denied.
- `as` is denied, except on a site whose `#[expect]` says why the conversion cannot lose data.
- nishiki forbids unsafe code.
  Test support that cannot be written safely lives in its own unpublished workspace crate, as [Google's Rust style guide](https://google.github.io/styleguide/rustguide.html) asks for unsafe code: behind a safe API, in a separate crate with few dependencies.
  Its crate docs justify the unsafe code, every unsafe block carries a `SAFETY:` comment, and its tests run under Miri.
  The one such crate is `nishiki-alloc-count`, a counting global allocator for the test that proves 2.1 and for the memory counts of `examples/memory.rs` (operator, 2026-11-03).

### 3.4 Comments carry the why

- Doc comments say why, on the item.
- Comments are plain prose, as `AGENTS.md` "Documentation" item 3 sets out: short sentences, one claim each, concrete names.
  A comment that needs a second read is rewritten in review.
- Inline prose inside a body is removed in review.
- Multiline text in code and tests uses `indoc!` or `formatdoc!`.
  `check:literals`, which the pre-commit hook and `check:local` run, fails on a string literal in `src/` or `tests/` that holds two or more escaped line feeds, outside comments and generated files.
  A literal of line feeds alone, such as a `"\n\n"` separator, passes.

### 3.5 Dependencies

- The crate depends on `memchr` alone.
- A feature may add one dependency when it removes more work or risk than it adds,
  and the change says why.
  `json` adds `serde_json`.
  The engine's hidden `_textmate` feature adds `regex-automata` and `regex-syntax`.
  `regex-automata` already depends on `regex-syntax`.
- A dependency is pure Rust.
  One that builds C or carries unsafe code needs a recorded reason.

### 3.6 Public API

The API follows the Rust API guidelines:

- `new` and `new_with_*` constructors;
- getters by value only for `Copy` types;
- enums rather than `bool` parameters;
- `#[non_exhaustive]` where variants will grow.

A change that breaks the API is called out in its changeset.
The release rules in `AGENTS.md` decide its version.

### 3.7 Two kinds of lexer, two kinds of loop

- A hand-written lexer keeps its own loop, budget and state.
  It follows the contract in 1.1 to 1.6 and needs nothing from `src/rules.rs`.
- A language lexed from a TextMate grammar's generated tables uses `src/rules.rs`.
  - The per-line loop, `\G`, capture groups, frames, budget checks and the candidate tables for each byte exist once there.
  - The language supplies a `Grammar` implementation: its generated tables, its scope type, its `SOURCE` scope and its hand-written matchers.
  - Its module's `start`, `start_embedded` and `lex` call `rules::start`, `rules::start_embedded` and `rules::lex`, which hold the contract of 1.1 once.
  - It also supplies an `Extra`, for what its matchers remember about a line.
  - It sets `Grammar::GROUPS` to one more than the highest capture group its patterns record, and its `lang-*` feature enables `_rule-tables` when it scans with `table_scan`.
  - Its generator, `oracle/<language>-grammar.ts`, takes the first-byte analysis, the start table, the rule codes with the `Tabled` implementation, the scope file and the `GROUPS` check from `oracle/rule-grammar.ts`; the generated grammar fails to compile when `GROUPS` is short of a group its tables name.
  - It writes every Rust file through `writeRust` there.
    `writeRust` formats the file as `mise run format` does, so a run leaves the committed tables unchanged.
    `mise run check:rule-grammars` checks that round trip in CI for every such language.
  - A match's groups come from the pattern's table, or from spans the matcher records (`span_one`, `span_dyn` for a scope named by text of the line); `Lexer::retokenise` lexes a span again with another rule and serves a grammar's `tokenize_nested` hook.
  - `src/python/engine.rs`, `src/java/engine.rs`, `src/css/engine.rs`, `src/kotlin/engine.rs` and `src/make/engine.rs` show the shapes.
  - Such a language does not copy the loop.
- A language with no lexer of its own runs on nishiki's `TextMate` engine (`src/textmate.rs`), which interprets Shiki's grammar as Shiki does.
  - Its row in `languages!` (`src/language.rs`) names its table as `engine: crate::<language>::grammar::GRAMMAR`, and the three entry points call `crate::textmate` with it.
  - Its module `src/<language>.rs` holds only its doc comment and `pub(crate) mod grammar;`, the tables `oracle/engine-tables.ts` writes into `src/<language>/grammar.rs`.
  - Its scopes are named at run time, so it has no scope table, frame tag or `Scratch` field of its own.
  - Its `lang-*` feature turns on `_textmate` and sits in the `textmate` tier, not in `all`, so the default build carries no grammar tables.
  - The two engines are different things: `src/rules.rs` runs hand-written matchers over generated tables, and the `TextMate` engine runs the grammar's own regular expressions.

### 3.8 Shared primitives

- Before writing a helper, a lexer checks the shared modules and uses them:
  `src/classes.rs`, `src/utf8.rs`, `src/space.rs`, `src/rules.rs`, and the arena and scope helpers.
- A helper a second lexer needs moves to a shared module named for its concern.
  It moves in the pull request that needs it, and is never copied.
- Lexers are written in parallel, so this is how duplication stays out.

## 4. Tests

### 4.1 Shiki is the test

- A language is done when every oracle fixture matches Shiki on every non-whitespace byte,
  in scopes and in colours under every oracle theme, held-out inputs included.
- Unit tests cover what the oracle cannot reach:
  states across lines, limits, embedding, and edge cases a fixture lacks.

### 4.2 The bar for a new language

Before its pull request opens, a language has:

- [ ] development and held-out oracle inputs, with their licences in `tests/oracle/SOURCES.md`;
- [ ] its grammar's notice in `THIRD-PARTY-NOTICES.md`;
- [ ] the colour test;
- [ ] its hostile lines in `tests/it/limits/<language>.rs`, a new file there;
- [ ] a pass of the fuzz test;
- [ ] its `mise run bench` table.

### 4.3 One integration binary

Integration tests are modules of `tests/it/main.rs`.
A new file is a module there, not a new binary.

## 5. Crates and publishing

### 5.1 One published crate until a real boundary

- nishiki is one published crate.
- A part becomes its own crate only at a real boundary,
  shown by how often it changes and who depends on it, or by a build-time measurement.
  The likely first is the TextMate engine's bundled grammar data, whose size would rebuild everything else.
- Crates are named `nishiki-<noun>`, never with `-rs`.

### 5.2 Languages are features

- Each lexer module is a `lang-*` feature, in the `all` default and in the tiers that fit it.
  One module that lexes several languages (TypeScript's four dialects) gets a feature for each.
- A language that embeds another lists it as a feature dependency.
- Code that only some languages use sits behind a hidden feature they turn on (`_classes`, `_utf8`, `_space`, `_rule-engine`).
- Wiring a language for features means a `cfg` on each of: its `mod` in `src/lib.rs`, its row in `src/language.rs`, its constant in `src/tag.rs`, its line in `src/scope.rs`, its fence names, and its `Scratch` field.
- A fence of a language that is off is plain text, as in Shiki when that grammar is not loaded.
- `check:features` builds each language, each tier and the empty set alone.
  It denies every warning except dead code: a language built without the ones that embed it leaves some embedding code unused, which no set avoids.
  The empty set and `json` alone deny every warning, dead code included.
  The code every lexer shares sits behind the private `_language` feature, which every `lang-*` feature and the TextMate engine turn on.
- Unit tests that name another language are gated on its feature.

### 5.3 What ships

- `include` in `Cargo.toml` lists what a published crate ships.
  Tests, fixtures, the oracle, benchmarks and docs never ship.
- A workspace member that is not for crates.io sets `publish = false`.
- The integration test builds in `tests/`, and the examples in `examples/`, two such members.
  The published package then declares no target whose file `include` leaves out, and `cargo package` warns of none.
  `.ctl/ver.yaml` lists only the packages that publish.
- Measurement and research programs live in their own workspace, excluded from the root,
  so the root's lints, deny rules and versions never read them.
