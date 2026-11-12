# Coding standards

Numbered rules for nishiki's code.
Reviews cite the number.

- `AGENTS.md` holds the invariants these rules serve.
- The lints in `Cargo.toml` and `clippy.toml` enforce what a machine can check, each with its reason.
- This file states only what nishiki adds or decides.
  The general rules come from:
  - the `rust-projects` skill's code rules;
  - the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/);
  - [Google's Rust style guide](https://google.github.io/styleguide/rustguide.html), its general rules, leaving out the Chrome, Fuchsia and Android ones;
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
5. its `lang-*` feature in `Cargo.toml`, which turns on `_hand-lexers`, with a `cfg` on each of the above (5.2).

The scope tables, the frame tags and the `lang-*` features are sorted by language.
So are the other lists a language adds to:

- the README's language table;
- `src/scratch.rs`, `mise-tasks/check/features.sh`, `oracle/inputs.ts` and `tests/oracle/SOURCES.md`;
- Markdown's fence languages and fence scopes, in `src/markdown/rules.rs` and `src/markdown/scope.rs`;
- the sections of `THIRD-PARTY-NOTICES.md`.

A new row goes in its sorted place, never at the end, so two languages added in parallel edit different lines.
`tests/it/sorted.rs` fails when a list is out of order.

The rows of `src/language.rs` stay in the order they were added, with a new row at the end, until the next minor release sorts them.
Their order sets the discriminants of the public `Language` enum and the order of `Language::ALL`.
Moving one is therefore an API break.

A language has no highlighter, interner or state type of its own.
One contract keeps every language reachable through one public API,
and lets one language embed another on the same arena.

### 1.2 State lives in the arena

- A lexer's open contexts are frames of the shared arena (`src/arena.rs`).
  They are pushed and popped through the lexer's `push`, `push_with`, `pop`, `set_carried` and `carried`.
- A frame's `rule` names the open context, as a variant of the language's enum tagged with its own high byte.
- Every tag is a constant in `src/tag.rs`.
  A compile-time check fails when two ranges overlap,
  including a lexer that claims a wider range for flag bits.
- A lexer takes its constant there, never the next free byte, because lexers are written in parallel.
- Data a context carries is the frame's `carried` value:
  a raw string's `#` count, a fence's delimiter, an embedded language's state.
  Text is interned first, so the value is an id.

Equal states then have equal ids.
That is what lets an editor stop re-lexing as soon as a line's state matches the old one.

### 1.3 Scope names are tables

- Each language lists its scope names once, in Shiki's spelling,
  as an enum whose discriminant indexes a `NAMES` table.
- `src/scope.rs` numbers every table in one space.
  A table's slot is the high byte of its language's frame tag.
  A scope number is the slot in the high 16 bits and the name's index in the low 16.
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
- Its state is the carried value of the fence's frame.
- It spends the embedding line's work budget.
  - Each lexer counts in a `Steps` field of its own, against the limit its `Limits` give it.
  - The embedding lexer passes it limits whose work is what the line has left (`Steps::lend`).
  - Every lexer's `lex` returns what it spent beside its state (`Finished`).
  - The embedding lexer adds that to its own count (`Steps::take_back`).
- It counts only its own contexts against the depth limit.
  A pinned depth then gives it the same output wherever it is embedded.
- A fence's `Container` holds the embedded `Language`.
  Markdown numbers the `Container` codes of fences from `Language::ALL`.
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

- When fidelity and speed cannot both hold, the design is wrong (operator, 2026-11-04).
  Do not treat the loss as a price to pay.
  Rework the data layout or the point where the work happens until both hold.
- A cost accepted to merge a change, such as a language over the Callgrind gate, is temporary.
  The same pull request names the follow-up that removes it, and that follow-up is planned work.

### 2.5 A lazy cell's closure captures by value

- On a hot path, the closure given to `get_or_init` is `move`.
  It captures only `Copy` values, such as an index or a shared reference.
- A closure that borrows its captures builds their environment on the stack before the cell's initialized test.
  It does so on every call, though it runs only once.

### 2.6 State on the hot path has no `Drop` impl

- A value a lexer holds while it lexes a line, such as its `Steps`, has no `Drop` impl.
  Drop glue turns every call made while the value is alive into an unwinding call with a cleanup path.
  That path grows the caller and changes what LLVM inlines into it.
- Work that must happen when the value is done goes in one consuming method.
  The type system makes every exit call it.
  For example, `Steps::finish` is the only way to make the `Finished` that carries a lexer's state.

## 3. Code

### 3.1 Module layout

- A module with submodules lives in `foo.rs` beside `foo/`, never `mod.rs`.
- A file is named for the one concern it owns (`strings.rs`, `heredoc.rs`).
  Names such as `misc.rs`, `utils.rs` or `decl2.rs` name no concern and are refused in review.
- An item is `pub(crate)` unless the public API needs it.

### 3.2 No panics in library code

- Library code returns typed values or errors and never panics.
- An invariant is an exhaustive `match` with no `_` arm, or a typed error.
- Tests may unwrap.

### 3.3 Exceptions are local and explained

- A lint exception is `#[expect(lint, reason = "…")]` on the smallest item.
  `#[allow]` is denied.
- `as` is denied, except on a site whose `#[expect]` reason states why the conversion cannot lose data.
- nishiki forbids unsafe code.
- Test support that cannot be written safely lives in its own unpublished workspace crate.
  [Google's Rust style guide](https://google.github.io/styleguide/rustguide.html) sets the same rule for unsafe code: behind a safe API, in a separate crate with few dependencies.
  - Its crate docs justify the unsafe code.
  - Every unsafe block carries a `SAFETY:` comment.
  - Its tests run under Miri.
- The one such crate is `nishiki-alloc-count`, a counting global allocator (operator, 2026-11-03).
  It serves the test that proves 2.1 and the memory counts of `examples/memory.rs`.

### 3.4 Comments carry the why

- Doc comments state why, on the item.
- Comments are plain prose, as `AGENTS.md` "Documentation" item 3 sets out: short sentences, one claim each, concrete names.
  A comment that needs a second read is rewritten in review.
- Inline prose inside a body is removed in review.

### 3.5 Dependencies

- The crate depends on `memchr` alone.
- A feature may add one dependency when it removes more work or risk than it adds,
  and the change states why.
  `json` adds `serde_json`.
  The engine's hidden `_textmate` feature adds `regex-automata` and `regex-syntax`.
  `regex-automata` already depends on `regex-syntax`.
- A feature for build scripts may add the crates that write Rust.
  `codegen` adds `proc-macro2` and `quote`, and `format` adds `prettyplease` and `syn`.
  Each takes `default-features = false` and only the features it uses.
  A program that includes the generated Rust needs none of them.
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
  - Its generator, `oracle/<language>-grammar.ts`, takes five parts from `oracle/rule-grammar.ts`:
    the first-byte analysis, the start table, the rule codes with the `Tabled` implementation, the scope file and the `GROUPS` check.
    The generated grammar fails to compile when `GROUPS` is short of a group its tables name.
  - It writes every Rust file through `writeRust` there.
    `writeRust` formats the file as `mise run format` does, so a run leaves the committed tables unchanged.
    `mise run check:rule-grammars` checks that round trip in CI for every such language.
  - A match's groups come from the pattern's table, or from spans the matcher records.
    The matcher records them with `span_one`, or with `span_dyn` for a scope named by text of the line.
    `Lexer::retokenise` lexes a span again with another rule and serves a grammar's `tokenize_nested` hook.
  - `src/python/engine.rs`, `src/java/engine.rs`, `src/css/engine.rs`, `src/kotlin/engine.rs` and `src/make/engine.rs` show the shapes.
  - Such a language does not copy the loop.
- No language runs on nishiki's `TextMate` engine (`src/textmate.rs`): every language has a lexer of its own.
  - The engine runs a grammar a caller brings, through the grammar API that the `textmate` feature builds.
  - `src/textmate/compile/` links a grammar's JSON into the engine's tables when the caller loads it.
  - `oracle/engine-tables.ts` writes the same tables as JSON, as the oracle for that compiler.
  - The engine interprets a grammar as Shiki does, so it also checks the hand-written lexers.
  - The two engines are different things: `src/rules.rs` runs hand-written matchers over generated tables, and the `TextMate` engine runs the grammar's own regular expressions.

### 3.8 Shared primitives

- Before writing a helper, a lexer checks the shared modules and uses them:
  `src/classes.rs`, `src/utf8.rs`, `src/space.rs`, `src/rules.rs`, and the arena and scope helpers.
- A helper a second lexer needs moves to a shared module named for its concern.
  It moves in the pull request that needs it, and is never copied.
- Lexers are written in parallel, so this is how duplication stays out.

### 3.9 Names are prose

The prose rules in `AGENTS.md` cover identifiers too (operator, 2026-11-10).
This holds for types, variants, fields, functions, locals, modules, files and tests.

- A name is the plainest word a reader of the domain would use for its value.
- A name never uses a word that fits anything:
  `Kind`, `Type`, `Info`, `Data`, `Item`, `Value`, `Helper`, `Util` or `Manager`.
- A name never uses a coined word, or a rare word where a common one does.
- To name a set of variants or fields, ask what they have in common, and name that.
  - `LineKind` became `Freshness`: its variants mark whether a line's tokens are final or may still change.
  - `TagInfo` became `TagClass`: it holds the category and the rule the HTML grammar gives a tag.
- One concept keeps one word across languages.
  Every language's frame enum is its `Rule`, and a frame's field is `rule`.
- Names other crates and Shiki spell stay as they are, such as `HirKind` or a scope name.
- A scope id abbreviates the scope string in its `scope_table!` row, and the row is its definition.

### 3.10 Sources read without decoding

Every file a reader opens reads as it stands: no line to scroll, no escapes to decode (operator, 2026-11-10).

- Multiline text in code and tests uses `indoc!` or `formatdoc!`.
  In the Bun scripts it uses `dedent`, from `oracle/dedent.ts`.
  - `check:literals` fails on a string literal with two or more escaped line feeds and some text.
    It reads all the Rust and the Bun scripts.
  - A literal of line feeds alone, such as a `"\n\n"` separator, passes.
- No Rust string literal continues on the next line through a backslash line break.
  - A lint reason or a message fits on one line.
    A reason's detail goes in the item's doc comment.
  - `check:literals` fails on such a literal, wherever the Rust sits.
  - The pre-commit hook runs it when a commit touches a Rust file, and `check:local` runs it in CI.
- A file the repository writes is formatted by its writer.
  A check compares the committed file with the writer's output, as `check:engine-grammars` does for the grammar JSON.
- A line holds at most 100 characters in `.rs`, `.ts`, `.sh`, `.toml`, `.yml`, `.yaml` and `.json` files.
  `check:line-width` counts characters, and `verify` runs it.
  - rustfmt formats the Rust, and oxfmt the TypeScript, both at 100 columns.
    `mise run format:ts` checks the TypeScript, with the settings in `.oxfmtrc.json`.
  - oxfmt is the formatter the house Copier template uses, so nishiki stays in step with the template.
  - Neither formatter breaks a long string or comment.
    A comment is broken by hand, one sentence per line.
    A long string is shortened, split into separate strings, or written as a document.
- A line that cannot break is exempt by what it holds, never by a list of files:
  - a URL, or an action pinned to a commit;
  - an import or a regex literal in TypeScript;
  - a Markdown table row;
  - a matcher's regex in the docs of a matchers module, in a `text` block or a code span, written with no whitespace;
  - a JSON string.
- `bench/corpus/`, `oracle/inputs/` and `tests/oracle/` hold language text and the oracle's output, outside the bound.
- Two exemptions are temporary, each until the change that removes it:
  - the Bun scripts that write Rust, and the files they write, until rule-table-codegen moves each writer to Rust;
  - the Swift matcher names, until the Swift generator moves to `codegen/` (rule-table-codegen design 2.4).
- A list puts one entry per line.
  Only a table of short plain values packs several entries on a line, such as byte ranges or the word tables that `codegen/src/numbers.rs` fills.

### 3.11 Rust writes generated Rust

Every generated Rust file is written by `codegen/`, a crate in a workspace of its own (operator, 2026-11-10).

- A Bun script under `oracle/` only reads facts from Shiki or Oniguruma.
  It records them as formatted JSON under `oracle/facts/`, and a Rust writer reads them.
- `codegen/` reads grammars from `tests/engine/grammars/` and parses them through `nishiki::unstable`, the surface a caller has.
  It never parses a grammar or an expression a second time.
- `mise run codegen` writes each file whose bytes change.
  `mise run codegen -- --check` in `verify` fails when one would change, and names the command.
- A writer lays out its own output and never calls the system rustfmt, so a run writes the same bytes on every machine.
  - Structured code is built with `quote` and laid out by `prettyplease`.
  - A numeric or word table is filled to 100 columns by `codegen/src/numbers.rs`.
  - Every map a writer walks is a `BTreeMap`, or is sorted first.
- A generated file opens with the header of `codegen/src/header.rs`: the writer, the input and `mise run codegen`.
  Its `//!` docs follow.
- The `mod` line that includes it carries `#[rustfmt::skip]`.
  The `@generated` marker alone does not stop rustfmt.
- A generated file is never edited by hand.
  A change goes in its writer, or in the facts or grammar it reads.
- A lint the output trips gets `#[expect]` on the smallest item, as 3.3 says.

Until [rule-table-codegen](openspec/changes/rule-table-codegen/tasks.md) moves them, the `oracle/<language>-grammar.ts` scripts still write the rule-engine tables and 13 `words.rs` files (3.7).
A generator moves to `codegen/` in one pull request, as the seven word scripts did:

1. The grammar it reads goes into `tests/engine/grammars/` through `oracle/engine-grammars.ts`, with its licence in `oracle/licences.ts`.
2. A module under `codegen/src/` writes the file, such as `codegen/src/words/sql.rs`.
   Its parent module adds it to the files it returns.
3. `mise run codegen` writes the file.
   The diff keeps every entry and token, in order; only the layout and the header change.
4. The language's `mod` line gains `#[rustfmt::skip]`.
5. The same pull request deletes the script and every check, workflow job and README row that ran it.
6. `codegen:check` then holds the file in `verify`.

## 4. Tests

### 4.1 Shiki is the test

- A language is done when every oracle fixture matches Shiki on every non-whitespace byte.
  It matches the scopes and the colours under every oracle theme, held-out inputs included.
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
A new file adds a module to it and builds no new binary.

## 5. Crates and publishing

### 5.1 One published crate until a real boundary

- nishiki is one published crate.
- A part becomes its own crate only at a real boundary.
  How often it changes and who depends on it show that boundary, or a build-time measurement does.
  The likely first is the TextMate engine's bundled grammar data, whose size would rebuild everything else.
- Crates are named `nishiki-<noun>`, never with `-rs`.

### 5.2 Languages are features

- Each lexer module is a `lang-*` feature, in the `all` default and in the tiers that fit it.
  One module that lexes several languages (TypeScript's four dialects) gets a feature for each.
- A language that embeds another lists it as a feature dependency.
- Code that only some languages use sits behind a hidden feature they turn on (`_classes`, `_utf8`, `_space`, `_rule-engine`).
- Wiring a language for features means a `cfg` on each of these:
  - its `mod` in `src/lib.rs`;
  - its row in `src/language.rs`;
  - its constant in `src/tag.rs`;
  - its line in `src/scope.rs`;
  - its fence names;
  - its `Scratch` field.
- A fence of a language that is off is plain text, as in Shiki when that grammar is not loaded.
- `check:features` builds each language, each tier and the empty set alone.
  - It denies every warning except dead code.
    A language built without the ones that embed it leaves some embedding code unused, which no set avoids.
  - The empty set, `json` alone and `textmate` alone deny every warning, dead code included.
- The code every lexer shares sits behind the private `_language` feature.
  Every `lang-*` feature and the TextMate engine turn it on.
- Every `lang-*` feature turns on the private `_hand-lexers` feature, which turns on `_language`.
  The TextMate engine alone leaves the code only hand-written lexers use unused.
  That code has `expect(dead_code)` under `not(feature = "_hand-lexers")`.
- Unit tests that name another language require its feature.

### 5.3 What ships

- `include` in `Cargo.toml` lists what a published crate ships.
  Tests, fixtures, the oracle, benchmarks and docs never ship.
- A workspace member that is not for crates.io sets `publish = false`.
- The integration test builds in `tests/`, and the examples in `examples/`, two such members.
  The published package then declares no target whose file `include` leaves out, and `cargo package` warns of none.
  `.ctl/ver.yaml` lists only the packages that publish.
- Measurement and research programs live in their own workspace, excluded from the root,
  so the root's lints, deny rules and versions never read them.
