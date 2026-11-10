# Design

## 1. What writes Rust today

### 1.1 The rule-engine scripts

On main at 851351f0, the 48 per-language scripts hold 16,250 lines, as `wc -l` counts them.
Each line was put in one class.
The class came from its top-level statement and from how many scripts hold the same line once the language name is masked.

| Class | Lines | What it is |
|:--|--:|:--|
| Comments and blank lines | 3,629 | file docs, section comments |
| Imports and types | 1,301 | `Node`, `Pattern` and `Rule` types, copied |
| Shared emitter copies | 7,857 | `scopeIdent`, the scope registry, `register`, `flatten`, `expand`, the `grammar.rs` template |
| Emitter variants | 1,164 | output only some languages write, such as C#'s `How` table and Swift's `dispatch.rs` |
| Data | 762 | renames, externals, end halts, word-list sources, prefixes |
| Logic | 1,537 | the rest |

The logic sits in a few scripts: C# 202 lines, Astro 127, Perl 127, Swift 105, Svelte 99, C++ 92.
Nine scripts have none, and 22 have 20 lines or fewer.
Most of it is four features, each written several times:

- the split of a `$N` scope name into one pattern per alternative, in 12 scripts;
- `while` rules, in 10;
- nested capture lists, in 16;
- parsers that find a capture group's text in a regex.

The shared files are [`oracle/rule-grammar.ts`][rule-grammar] (994 lines), [`oracle/rule-generator.ts`][rule-generator] (803) and `oracle/php-family-grammar.ts` (801).

### 1.2 Every script, by oracle

A part that asks Shiki or Oniguruma stays JavaScript.
A part that only reads grammars and writes Rust moves to Rust.

| Script | Writes | Stays JavaScript | Moves to Rust |
|:--|:--|:--|:--|
| `unicode-tables.ts` | `src/textmate/regex/unicode.rs` | one Oniguruma scan per set, recorded in `unicode-sets.json` | the file |
| `case-folds.ts` | `src/textmate/regex/folds.rs` | Oniguruma's orbits and string folds, recorded in `case-folds.json` | `NEXT`, `LARGEST_ORBIT`, `MULTI`, `UNFOLD`, the `WIDE_*` tables, and the file |
| `classes.ts` | `src/classes/table.rs` | the five sets read by tokenizing (`\d`, `\w`, `\s`, `[[:alpha:]]`, `\p{upper}`), recorded in `classes.json` | the file; its `\p{…}` unions come from `unicode-sets.json` |
| `engine-tables.ts` | JSON tables for the compiler's survey test | all of it: it writes no Rust | nothing |
| `fence-stops.ts` | nothing: it checks `Language::pops_fence` | nothing: it reads grammars only | all of it, as a check in `codegen/` |
| 7 word scripts | `src/{c,coffee,css,html,less,python,sql}/words.rs` | nothing: they read grammars only | all of it |
| 48 grammar scripts | each language's `grammar.rs`, `scope.rs` and the files of section 8 | nothing: `RX_DUMP` only writes expressions for `onig.ts` | all of it; `codegen --expressions` writes the list `onig.ts` reads |
| `fuzz.ts` and its 21 data files | test fixtures, through Shiki | all of it | nothing |

`classes.ts` today measures the `\p{…}` unions a second time.
[`CODING_STANDARDS.md`][standards] 3.8 gives each primitive one home, so the unions come from `unicode-sets.json`.

## 2. Rules the design follows

| Rule | Where it applies |
|:--|:--|
| [`CODING_STANDARDS.md`][standards] 3.1, one concern per module, `foo.rs` beside `foo/`, `pub(crate)` | sections 4 and 5 |
| 3.2, no panics, exhaustive matches | sections 5 and 9 |
| 3.3, no `as` | the table model numbers rules and patterns with `u8` and `u16` through `TryFrom` |
| 3.4, the why in doc comments, no `//` in bodies | every module |
| 3.5, no dependency without a reason | section 10 |
| 3.6, public API | section 5 |
| 3.8, one home per shared primitive | sections 1.2, 5.3 and 8 |
| 3.9, names are prose | the names in sections 4, 5 and 8 |
| rust-projects code rules 4.1, invalid states unrepresentable | the table model's enums, section 8 |
| code rules 5.1 to 5.3, one typed error per operation | section 9 |
| lints 19, one home per concept | the grammar model and the regex parser are shared, not copied |

## 3. Where generation runs

The [codegen research][research] read how mature crates generate committed Rust.

- A build script must not write outside `OUT_DIR`, by the [Cargo book][cargo-build-scripts].
  So committed files come from a tool a person or CI runs.
- [syn][syn-codegen] writes `src/gen/` with `codegen/`, a crate with `publish = false`.
  [unicode-ident][unicode-ident-write] writes `src/tables.rs` with `generate/`.
  [rust-analyzer][ra-codegen] runs `cargo codegen --check` in CI.
  Each CI job regenerates the files and fails on `git diff --exit-code`.
- nishiki follows syn: `codegen/` is a crate named `nishiki-codegen`, with `publish = false`, in a workspace of its own.
  - Its manifest has its own `[workspace]`, its own `Cargo.lock` and its own `target/`.
    The root manifest lists `codegen` under `exclude`, beside `bench`.
  - It keeps its own dependencies, such as `serde_json` for the facts, out of nishiki's manifest.
  - `mise run codegen` runs it, and `mise run codegen -- --check` fails when a file would change.
  - CI runs the check in `verify`, offline, from `tests/engine/grammars/` and `oracle/facts/`.
    Its error names the command that regenerates the files.
  - It writes a file only when its bytes change, as syn and unicode-ident do.
- The crate builds with `--cfg nishiki_unstable` and nishiki's `codegen` and `format` features.
  It writes the rule-engine tables through `nishiki::unstable`, the surface a caller has.
  So the internal generator and the public one are one code path.
- A workspace of its own keeps both settings away from the root workspace:
  - A member's features unify across a workspace build.
    As a member, `codegen/` would turn `codegen` and `format` on for every root build, `check:size` included.
  - The cfg sits in `codegen/.cargo/config.toml`, for `cargo` and rust-analyzer run in `codegen/`.
    Cargo reads that file from the directory it runs in, and the mise task runs in `codegen/`.
    Its builds then use `codegen/target/`, so they never rebuild the root's target directory.
  - A set `RUSTFLAGS` or `CARGO_ENCODED_RUSTFLAGS` replaces `build.rustflags` ([Cargo book][cargo-rustflags]).
    A caller or a CI job may set `RUSTFLAGS`, as `check:features` does for its own builds.
    So `mise run codegen` appends `--cfg nishiki_unstable` to `RUSTFLAGS` and keeps the caller's flags.
    It fails when `CARGO_ENCODED_RUSTFLAGS` is set, because that variable replaces `RUSTFLAGS` too.
  - `cargo deny` runs on its manifest too, in `verify`.

## 4. The `codegen/` crate

`codegen/src/main.rs` beside `codegen/src/`, each file one job:

| Module | Job |
|:--|:--|
| `main.rs` | the command line: write, `--check`, `--expressions <file>`, `--compare <base>` |
| `facts.rs` | reads `oracle/facts/*.json` into typed values, once, at the boundary |
| `numbers.rs` | the line-filling writer for numeric tables |
| `header.rs` | the `// @generated` header each file opens with |
| `unicode_sets.rs` | writes `unicode.rs` from `unicode-sets.json` |
| `case_folds.rs` | derives the fold tables from `case-folds.json` and writes `folds.rs` |
| `classes.rs` | writes `table.rs` from `classes.json` and `unicode-sets.json` |
| `words.rs` | writes each `words.rs` from the alternatives of a grammar's expressions |
| `rule_tables.rs` | calls `nishiki::unstable::codegen` for each rule-engine language |
| `languages.rs` beside `languages/` | one value per rule-engine language, section 8.2 |
| `fence_stops.rs` | the check `oracle/fence-stops.ts` runs today |
| `compare.rs` | the token comparison of section 11 |

## 5. nishiki's features and public surface

### 5.1 Features

| Feature | Dependencies | What it adds |
|:--|:--|:--|
| `codegen` | `proc-macro2`, `quote` | `ToTokens` for `Grammar`, and the code writer under `nishiki::unstable` |
| `format` | `codegen`, `prettyplease`, `syn` | `Grammar::to_rust`, a formatted file |

Neither is on by default, and neither is in `all`.
A caller turns them on for a build script's dependency, so the program's own build gains nothing.
[prost-build][prost-format] has the same split: tokens always, formatting behind `format`.
The operator chose these dependencies on 2026-11-10.

### 5.2 A caller's grammar as Rust

- `impl quote::ToTokens for Grammar`, with `codegen`.
  The tokens are an expression of type `Grammar`, built in a `static` with no work at run time.
  A build script writes them to `OUT_DIR`, and the program reads them with `include!`, as the [Cargo book][cargo-build-scripts] shows.
- `Grammar::to_rust(&self) -> String`, with `format`.
  It borrows, as the API guidelines' `to_` prefix does ([C-CONV][c-conv]), so a caller can keep using the grammar.
  It returns a file that holds `pub static GRAMMAR: nishiki::Grammar = …;`, laid out by prettyplease.
  The program writes `mod demo { include!(concat!(env!("OUT_DIR"), "/demo.rs")); }` and uses `demo::GRAMMAR`.
- Neither can fail, so neither has an `# Errors` section.
- Both get a doctest, and `examples/build-script/` gains a second program that uses `to_rust`.
- It saves the read of the bytes, which took 312 µs for TypeScript's grammar on a box (`bench:cold`).
  Linking, 2,565 µs there, still happens at the first `Highlighter::for_grammar`.

The tokens name the unlinked model's types.
They sit in a `#[doc(hidden)] pub mod __private`, outside the documented API.
The tokens are a block expression, `{ const _: () = ::nishiki::__private::FORMAT_1; <the Grammar value> }`.
A `static` initializer accepts a block that holds an item.
A program built with tokens from another nishiki version then fails to compile.
The missing name shows the cause, and no program runs with wrong tables.

The build-script example writes the tokens on every build, so its tables always match the nishiki it links.

Its build-dependency on nishiki takes `format`, and its normal dependency does not.
Cargo's version 2 feature resolver, which resolver 3 keeps, does not unify a build-dependency's features with a normal dependency's ([Cargo reference][cargo-resolver]).
So the example program, and `check:size`, build nishiki without them.

### 5.3 The unlinked model in a `static`

`compile::UnlinkedGrammar` holds `String` and `Vec` fields, which a `static` cannot hold.
Its strings become `Cow<'static, str>` and its lists `Cow<'static, [T]>`.

- The compiler and `from_bytes` build owned values, as today.
- The tokens build borrowed ones, and `Cow::Borrowed` is a `const` constructor.
- The linker reads both through `Deref`, unchanged.
- No second model exists, so the bytes format, the compiler and the tokens share one type.

This refactor is its own first pull request, with no behaviour change.

### 5.4 `nishiki::unstable`

The rule engine becomes public under a cfg, as tokio does with `tokio_unstable`:

- Cargo features unify across a dependency graph, so a cfg is the opt-in.
  In [tokio's docs][tokio-unstable-docs], the flag is how a caller opts in to APIs that may break semver.
- `nishiki::unstable` compiles only with `--cfg nishiki_unstable`.
  It holds the rule engine's `Grammar` trait, `Act`, the pattern definitions, the matcher signature, `RuleSyntax<G>` for `Highlighter<S: Syntax>`, and `unstable::codegen`.
- Its module docs state that it is exempt from semver and may change in any patch release.
  `AGENTS.md` "Releases" gains the rule: items under `nishiki::unstable` never force a minor.
- `[lints.rust] unexpected_cfgs = { level = "warn", check-cfg = ['cfg(nishiki_unstable)'] }`, as in [tokio's workspace manifest][tokio-check-cfg].
- `[package.metadata.docs.rs]` adds `rustdoc-args = ["--cfg", "docsrs", "--cfg", "nishiki_unstable"]` and `rustc-args = ["--cfg", "nishiki_unstable"]`, as in [tokio's manifest][tokio-docs-rs].
  Each item carries `#[cfg_attr(docsrs, doc(cfg(nishiki_unstable)))]`, as tokio's [`cfg_unstable_metrics!`][tokio-cfg-macro] does, so docs.rs shows the badge.
- The semver check runs without the cfg, as [tokio's semver job][tokio-semver] does.
- CI builds and tests once more with the cfg, so the unstable module stays compiled.

The rule engine's items stay in `src/rules.rs`, and `src/unstable.rs` re-exports them.
A `pub(crate)` item cannot be re-exported as `pub`, and `unreachable_pub` is denied.
So a declarative macro gives each item `pub` with the cfg and `pub(crate)` without it.

### 5.5 Default matchers

Today every pattern of a rule-engine grammar has a matcher written by hand.
A caller's grammar has none.

- `unstable::codegen` gives every pattern a default matcher.
  It runs the pattern's expression through the crate's TextMate regex engine, `src/textmate/regex`, compiled once on first use.
  That engine is built on `regex-automata`, and it adds what Oniguruma has and `regex-automata` lacks: back-references and lookbehind.
- A language can name a hand-written matcher for any pattern.
  The generated `matcher` calls it for that pattern and the default for the rest.
- nishiki's own languages name a hand matcher for every pattern, as today.
  A language can drop hand matchers where the default measures as fast.

How much slower the default is decides where it fits.
Task 1.1 measures it before any other code lands (section 12).

## 6. Facts

### 6.1 The files

Every fact file sits in `oracle/facts/`.
readable-sources-c9 writes the JavaScript side, and both sides agreed the format on 2026-11-10.

| File | Script | Holds |
|:--|:--|:--|
| `unicode-sets.json` | `oracle/unicode-tables.ts` | 73 sets, about 16,500 ranges, in the order `unicode.rs` lists them |
| `case-folds.json` | `oracle/case-folds.ts` | 1,427 orbits of more than one character, and 104 folds to a string |
| `classes.json` | `oracle/classes.ts` | the five sets read by tokenizing |

### 6.2 The format

- `JSON.stringify(value, null, 2)` and a trailing newline, keys in a fixed order.
- Every file has `"measured"`, such as `"Oniguruma, as shiki 4.6.0 bundles it"`, and `"script"`, the script that wrote it.
- A code point is a hex string in Unicode's notation: at least four digits, upper case.
  A range is `"0041..005A"`, and one point is `"00AA"`.
- `unicode-sets.json` and `classes.json`:

  ```json
  {
    "measured": "Oniguruma, as shiki 4.6.0 bundles it",
    "script": "oracle/unicode-tables.ts",
    "sets": [
      { "key": "l", "ranges": ["0041..005A", "0061..007A", "00AA"] }
    ]
  }
  ```

  A `posix_<name>` key is the bracket set that `lookup` serves under `<name>`.
- `case-folds.json`:

  ```json
  {
    "measured": "Oniguruma, as shiki 4.6.0 bundles it",
    "script": "oracle/case-folds.ts",
    "orbits": [["0041", "0061"], ["004B", "006B", "212A"]],
    "folds": [{ "char": "00DF", "fold": ["0073", "0073"] }]
  }
  ```

  Each orbit is ascending, and the orbits are sorted by their first member.
  `codegen/src/case_folds.rs` derives the tables `folds.rs` holds today.

### 6.3 The checks

- A fact file against its script: `check:unicode-tables`, `check:case-folds` and `check:classes` run the script and fail when the file changes.
  They run in the Generated tables workflow, which needs Shiki's packages.
- A generated file against its facts and grammars: `mise run codegen -- --check`, in `verify`.
- The header of each generated file names the command, the writer module and each input, so a reader finds both ends.

## 7. Writing the code

### 7.1 Two writers

| Output | Writer | Why |
|:--|:--|:--|
| Structured code: enums, `match` arms, rule tables, start tables | `quote`, laid out by `prettyplease` | syn and rust-analyzer build code this way, by the [research][research] |
| Numeric tables: Unicode ranges, folds, word blobs | `numbers.rs`, which fills each line up to 100 columns | prettyplease fills only arrays of plain literals ([`simple_array`][prettyplease-array]) |

- prettyplease's output depends only on its version.
  rustfmt's output depends on the toolchain, so the system rustfmt never lays out a committed file.
  ICU4X's CI notes that generated Rust is not stable across Rust versions ([research][research], "Where code generation runs").
- prettyplease breaks at its own margin, 89 columns, and writes one entry per line past it.
  The lines rustfmt gave up on today, up to 4,380 columns in C++, C# and Swift, are laid out by it.

### 7.2 Headers, formatting and lints

- Each file opens with `// @generated by mise run codegen. Regenerate with mise run codegen.`, as unicode-ident's does, then the `//!` docs that name the inputs.
- Its `mod` line carries `#[rustfmt::skip]`, as [unicode-ident's `lib.rs`][unicode-ident-write] does.
  The `@generated` marker alone does not stop rustfmt: `format_generated_files` is unstable and defaults to `true` ([rustfmt configuration][rustfmt-config]).
- A lint the generated code trips gets `#[expect(lint, reason = "…")]` on the smallest item, as [`CODING_STANDARDS.md`][standards] 3.3 asks.
- Every map the writers walk is a `BTreeMap`, or the writer sorts it first, so a run writes the same bytes every time.

## 8. The rule-engine tables

### 8.1 Model and parser

- `unstable::codegen` reads `compile::UnlinkedGrammar`, which `compile::compile` builds from the JSON once.
  It holds rules, includes, captures, `while`, `applyEndPatternLast` and nested repositories as typed values.
  No second JSON reader is written.
- It reads each expression with the crate's parser, `regex::parse::parse`, and its tree, `regex::ast::Node`.
  Both become `pub(crate)` in a pull request that moves nothing else.
- Its modules each do one job:

  | Module | Job |
  |:--|:--|
  | `flatten.rs` | includes and pattern lists, with vscode-textmate's drop rule |
  | `split.rs` | a `$N` scope name into one pattern per alternative |
  | `starts.rs` | section 8.3 |
  | `tables.rs` | the typed model: `Rule`, `Pattern`, `Capture`, `ScopeName`, `CarriedState` |
  | `names.rs` | identifiers such as `KwControl` and `RootAt3` |
  | `matchers.rs` | section 5.5 |
  | one writer per output file | its file |

- An act is `Match`, `Begin`, `End` or `While`, each with its own data, and every match on it is exhaustive.
- `CarriedState` is what a rule carries from line to line: nothing, the text of a begin group its end repeats, or an embedded language's state.
  The rule engine calls it `Payload` today, and task 2.3 renames it when it becomes public.

### 8.2 Per-language values

What a nishiki language needs beyond its grammar is one `RuleTables` value, in `codegen/src/languages/<language>.rs`:

| Field | Holds | Today |
|:--|:--|:--|
| `language`, `grammar` | the grammar file's name and scope name | each script's import |
| `module`, `engine`, `table` | the module path, the engine type and the scope table | the template's strings |
| `suffix` | the last part of every scope name, which identifiers drop | `scopeIdent` |
| `renames` | scope identifiers for names the grammar misspells | the `"pre" + "ffix"` splits |
| `externals` | each grammar another lexer takes, with its `Language` and feature | the `source.*` branches of `flatten` |
| `end_halts` | the halt of each embedding rule's end, by its expression | `endHalts` |
| `word_lists` | each list read from an expression, by pattern id and its bounds | `words` |
| `backref_start` | `BackrefStart::Quote` or `BackrefStart::Any` | `startTable`'s argument |
| `matchers` | `HandMatchers::All`, or the patterns with a hand matcher | every pattern, by hand |
| `names` | a name for each pattern whose derived name is too long (section 8.4) | the snake-cased pattern id |
| `added_outputs` | the files a language adds, each a function of the model | the six one-off outputs |

- The values are Rust, so the compiler checks every field, and no second file format exists.
- The six one-off outputs stay with their languages: C#'s `How` table, Swift's `dispatch.rs`, and C++'s, Julia's, Svelte's and CoffeeScript's.
  Each is a function that reads the typed model and returns tokens.
  The writer holds no branch on a language's name.
- A caller's `RuleTables` is the same type, from `nishiki::unstable::codegen`.

### 8.3 The first-byte analysis

- `starts.rs` ports `startOf`, `canMatchEmpty` and `takesOneBlank` from `oracle/rule-grammar.ts`.
  It reads the parser's tree instead of the expression's text.
- It computes the same facts:
  - the first-byte set, and the places a match can start;
  - whether it can match at the end;
  - whether it can take one blank.
- The tree folds `(?i)` characters into classes of their orbits, which the TypeScript reader does not.
  A set can then gain the first byte of a character such as the Kelvin sign.
  Such a set is still sound, but it differs.
- `codegen --compare` reports a differing set by its pattern.
  The pull request lists it, and Identity and Callgrind check it.
  The `generated-rust` spec allows such a set and no other token change.
- `oracle/rule-grammar.test.ts`'s 24 cases become unit tests with typed inputs.

### 8.4 Generated names

Generated identifiers follow rule 3.9, names are prose, and the 100-column limit, as written ones do (lead, 2026-11-10).
Swift's script breaks both today:

- It names each matcher `rx_` plus the snake-cased pattern id.
  The id spells the whole repository path.
- Of its 347 matcher names, 30 run 60 characters or more.
  The longest has 100: `rx_expressions_without_trailing_closures_or_member_references_function_or_macro_call_expression_at_1`.
- 143 lines that hold one pass 100 columns.

`names.rs` derives a short, plain name instead:

1. A pattern takes the repository key of its rule, the last part of the path.
   Its place in the rule follows: `begin`, `end`, `while`, or its position in a pattern list.
2. Two patterns with the same name are told apart by one rule.
   Each takes the nearest enclosing repository key where their paths differ.
   The whole path is never appended.
3. A name stays within 40 characters, so a line that calls it fits in 100 columns.
   A longer name is an error (section 9), and the language's `RuleTables` gives that pattern a name in `names`.

The same rule names every generated identifier, such as a frame's `Rule` variant.
A hand matcher takes the name of its pattern, so a move renames the hand-written function too.
`codegen --compare` maps each old name to its new one by pattern id, and compares the tokens after that.

The first Swift move proves the rule:

- its pull request renames Swift's matchers and their hand-written overrides;
- no generated Swift name passes 40 characters, and no line that holds one passes 100 columns.

The readability exemption for Swift's generated names ends with that pull request.

## 9. Errors

`unstable::codegen::GenerateError` is one enum, documented under `# Errors`:

- the grammar does not compile (`CompileError` as its source);
- an expression does not parse (the parser's error as its source);
- an include names no rule, and the language does not list it as external;
- a scope name repeats a group the expression lacks;
- the rules do not fit a byte, or the patterns a `u16`;
- an end repeats more than one group of its begin;
- a hand matcher names a pattern the grammar lacks;
- a derived name passes 40 characters, and `names` gives none (section 8.4).

Each variant names the pattern id or include that failed.
`codegen/` has its own error enum for a fact file it cannot read.
The stable surface of section 5.2 cannot fail.

## 10. Dependencies

| Crate | Feature | Version | Why |
|:--|:--|:--|:--|
| `proc-macro2` | `codegen` | 1.0.107 | the token type the public functions return |
| `quote` | `codegen` | 1.0.47 | builds tokens |
| `syn` | `format` | 3.0.10 | prettyplease reads a `syn::File` |
| `prettyplease` | `format` | 0.3.0 | lays out tokens as a file |

- `codegen` adds 3 crates: `proc-macro2`, `quote` and `unicode-ident`.
  `format` adds 2 more ([research][research], "Dependency cost").
- Default builds and `all` gain none.
- `codegen/` adds `serde_json` and `serde` with `derive`, for the facts, in its own manifest only.
  nishiki's own JSON reader is `pub(crate)` and built for grammars, so the facts use serde_json in `codegen/` alone.
- The size and build time of each feature are measured on a box (task 1.2), and the numbers go here.

## 11. The proof

- `mise run codegen -- --check` passes in `verify`.
- Each move is checked against its base commit in a build box, `8x16` or smaller, `--ttl 2h`, ephemeral:
  - `codegen --compare <base>` parses each changed file on both sides with `syn` and compares the tokens.
    Layout and `//` comments differ; tokens, doc comments included, do not.
  - Identity and `check:features` pass.
- A changed start set (section 8.3) is listed in the pull request, with its Callgrind rows.
- The output of the compare goes in the pull request body, under "How it was verified".
- `check:rule-grammars` runs the remaining scripts until the last one goes, then it is deleted.

## 12. Measurements

### 12.1 Default matchers

A prototype branch, `research/default-matchers`, swaps each hand matcher of Protocol Buffers, Zig and Lua for the default.
It is pushed with no pull request, so no CI runs.
A research note links it at a pinned commit (`openspec/AGENTS.md` rule 7).

Callgrind counts the lexing pass over each language's oracle fixtures, both ways, at one codegen unit (`bench/AGENTS.md` rule 12).
The numbers go here when they land.

### 12.2 Feature cost

On a box, from a clean target:

- the build time of nishiki with `all`, with `all,codegen` and with `all,codegen,format`;
- `cargo tree -e normal` for each, to count the crates;
- the size of `examples/build-script`'s program, which must not change.

The numbers go here when they land.

## 13. Order

1. The measurements of section 12.
2. `compile::UnlinkedGrammar` holds `Cow` data, and the regex parser becomes `pub(crate)`: no behaviour change.
3. The `codegen` and `format` features, `ToTokens` and `to_rust`, with their doctests and the example.
4. `nishiki::unstable` and its cfg setup, then `unstable::codegen` with default matchers, and `codegen/` writing Protocol Buffers' tables.
   The same pull request first adds Protocol Buffers' grammar to `tests/engine/grammars/` through `oracle/engine-grammars.ts`, with its licence in `oracle/licences.ts`.
   Every later move does the same for its own languages before `codegen/` reads them.
   `oracle/AGENTS.md` describes how to move one language.
   `AGENTS.md` and `CODING_STANDARDS.md` gain the rules of the proposal.
5. Unicode sets, case folds and classes, once readable-sources-c9's fact files land.
6. One pull request per family, each deleting its scripts and adding its grammars to `tests/engine/grammars/` through `oracle/engine-grammars.ts`.
7. The last family deletes `oracle/rule-generator.ts`, `oracle/rule-grammar.ts`, `oracle/php-family-grammar.ts` and `check:rule-grammars`.
8. The 7 word scripts and `fence-stops.ts` move into `codegen/`.

[cargo-resolver]: https://doc.rust-lang.org/cargo/reference/resolver.html#feature-resolver-version-2
[c-conv]: https://rust-lang.github.io/api-guidelines/naming.html#ad-hoc-conversions-follow-as_-to_-into_-conventions-c-conv
[cargo-build-scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[cargo-rustflags]: https://doc.rust-lang.org/cargo/reference/config.html#buildrustflags
[prettyplease-array]: https://github.com/dtolnay/prettyplease/blob/49e50897b580a5b7591ead6ef73f3fe09e76d896/src/expr.rs
[prost-format]: https://github.com/tokio-rs/prost/blob/b3f73911a2dca69a798fff418d35ed07e30edbc3/prost-build/src/config.rs
[ra-codegen]: https://github.com/rust-lang/rust-analyzer/blob/2a362a41a8eb6e090f1a12d9b712df85af6f0c33/xtask/src/codegen.rs
[research]: ../../research/2026-11-10-rust-codegen-practice.md
[rule-generator]: ../../../oracle/rule-generator.ts
[rule-grammar]: ../../../oracle/rule-grammar.ts
[rustfmt-config]: https://github.com/rust-lang/rustfmt/blob/main/Configurations.md#format_generated_files
[standards]: ../../../CODING_STANDARDS.md
[syn-codegen]: https://github.com/dtolnay/syn/blob/97ec37dda8a893a460066da694fd1eb7cb30538a/codegen/src/file.rs
[tokio-cfg-macro]: https://github.com/tokio-rs/tokio/blob/d6102c7e0a97b939c23d056122490faf777d2875/tokio/src/macros/cfg.rs#L278-L286
[tokio-check-cfg]: https://github.com/tokio-rs/tokio/blob/d6102c7e0a97b939c23d056122490faf777d2875/Cargo.toml#L28-L38
[tokio-docs-rs]: https://github.com/tokio-rs/tokio/blob/d6102c7e0a97b939c23d056122490faf777d2875/tokio/Cargo.toml#L179-L185
[tokio-semver]: https://github.com/tokio-rs/tokio/blob/d6102c7e0a97b939c23d056122490faf777d2875/.github/workflows/ci.yml#L504-L518
[tokio-unstable-docs]: https://github.com/tokio-rs/tokio/blob/d6102c7e0a97b939c23d056122490faf777d2875/tokio/src/lib.rs#L352-L374
[unicode-ident-write]: https://github.com/dtolnay/unicode-ident/blob/28018f288934fa6be60a6e791e071b7a7195643e/generate/src/write.rs
