# Design

This change moves every writer of generated Rust from TypeScript to a Rust crate, `codegen/`.
It holds the crate, where it runs, the fact files and the writers of numeric tables.
[The proposal][proposal] links the four changes split from it.

## 1. What writes Rust today

A part that asks Shiki or Oniguruma stays JavaScript.
A part that only reads grammars and writes Rust moves to Rust.

| Script | Writes | Stays JavaScript | Moves to Rust |
|:--|:--|:--|:--|
| `unicode-tables.ts` | `src/textmate/regex/unicode.rs` | one Oniguruma scan per set, recorded in `unicode-sets.json` | the file |
| `case-folds.ts` | `src/textmate/regex/folds.rs` | Oniguruma's orbits and string folds, recorded in `case-folds.json` | `NEXT`, `LARGEST_ORBIT`, `MULTI`, `UNFOLD`, the `WIDE_*` tables, and the file |
| `classes.ts` | `src/classes/table.rs` | the five sets read by tokenizing (`\d`, `\w`, `\s`, `[[:alpha:]]`, `\p{upper}`), recorded in `classes.json` | the file; its `\p{…}` unions come from `unicode-sets.json` |
| `engine-tables.ts` | JSON tables for the compiler's survey test | all of it: it writes no Rust | nothing |
| `fence-stops.ts` | nothing: it checks `Language::pops_host` | nothing: it reads grammars only | all of it, as a check in `codegen/` |
| 7 word scripts | `src/{c,coffee,css,html,less,python,sql}/words.rs` | nothing: they read grammars only | all of it |
| 48 grammar scripts | each language's `grammar.rs`, `scope.rs` and rule tables | nothing: `RX_DUMP` writes expressions for `onig.ts` | all of it; `codegen --expressions` writes the list `onig.ts` reads, in [rule-table-codegen][rule-table-codegen] |
| `fuzz.ts` and its 21 data files | test fixtures, through Shiki | all of it | nothing |

`classes.ts` today measures the `\p{…}` unions a second time.
[`CODING_STANDARDS.md`][standards] 3.8 gives each primitive one home, so the unions come from `unicode-sets.json`.

## 2. Rules the design follows

| Rule | Where it applies |
|:--|:--|
| [`CODING_STANDARDS.md`][standards] 3.1, one concern per module, `foo.rs` beside `foo/`, `pub(crate)` | section 4, and [grammar-to-rust][grammar-to-rust] |
| 3.2, no panics, exhaustive matches | [grammar-to-rust][grammar-to-rust], and the errors of [rule-table-codegen][rule-table-codegen] |
| 3.3, no `as` | the table model numbers rules and patterns with `u8` and `u16` through `TryFrom` |
| 3.4, the why in doc comments, no `//` in bodies | every module |
| 3.5, no dependency without a reason | the dependencies of [grammar-to-rust][grammar-to-rust] |
| 3.6, public API | [grammar-to-rust][grammar-to-rust] and [unstable-rule-engine][unstable-rule-engine] |
| 3.8, one home per shared primitive | section 1, the static model of [grammar-to-rust][grammar-to-rust], and the tables of [rule-table-codegen][rule-table-codegen] |
| 3.9, names are prose | the names in section 4, and those of [rule-table-codegen][rule-table-codegen] |
| rust-projects code rules 4.1, invalid states unrepresentable | the table model's enums, in [rule-table-codegen][rule-table-codegen] |
| code rules 5.1 to 5.3, one typed error per operation | the errors of [rule-table-codegen][rule-table-codegen] |
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
  - Its manifest has its own `[workspace]` and its own `Cargo.lock`.
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
    Cargo reads that file from the directory it runs in.
  - The mise tasks run from the root with `--manifest-path codegen/Cargo.toml`.
    They build in `target/codegen`, as `bench/` builds in `target/bench`, so they never rebuild the root's builds.
  - A set `RUSTFLAGS` or `CARGO_ENCODED_RUSTFLAGS` replaces `build.rustflags` ([Cargo book][cargo-rustflags]).
    A caller or a CI job may set `RUSTFLAGS`, as `check:features` does for its own builds.
    So `mise run codegen` sets `RUSTFLAGS` to the caller's flags and `--cfg nishiki_unstable`.
    It fails when `CARGO_ENCODED_RUSTFLAGS` is set, because that variable replaces `RUSTFLAGS` too.
  - `cargo deny` runs on its manifest too, in `verify`.

## 4. The `codegen/` crate

`codegen/src/main.rs` beside `codegen/src/`, each file one job:

| Module | Job |
|:--|:--|
| `main.rs` | the command line: write, `--check`, `--expressions <file>`, `--compare <base>` |
| `error.rs` | the error enum of every writer |
| `facts.rs` | reads `oracle/facts/*.json` into typed values, once, at the boundary |
| `numbers.rs` | the line-filling writer for numeric tables |
| `header.rs` | the `// @generated` header each file opens with |
| `unicode_sets.rs` | writes `unicode.rs` from `unicode-sets.json` |
| `case_folds.rs` | derives the fold tables from `case-folds.json` and writes `folds.rs` |
| `classes.rs` | writes `table.rs` from `classes.json` and `unicode-sets.json` |
| `words.rs` beside `words/` | writes each `words.rs` from the alternatives of a grammar's expressions, one module per language |
| `rule_tables.rs` | calls `nishiki::unstable::codegen` for each rule-engine language |
| `languages.rs` beside `languages/` | one value per rule-engine language, as [rule-table-codegen][rule-table-codegen] gives it |
| `fence_stops.rs` | the check `oracle/fence-stops.ts` runs today |
| `compare.rs` | the token comparison of [rule-table-codegen][rule-table-codegen]'s proof |

- `codegen/` adds `serde_json` and `serde` with `derive`, for the facts, in its own manifest only.
  nishiki's own JSON reader is `pub(crate)` and built for grammars, so the facts use serde_json in `codegen/` alone.
- `codegen/` has its own error enum for a file it cannot read or a grammar that lacks what a writer reads.

## 5. Facts

### 5.1 The files

Every fact file sits in `oracle/facts/`.
readable-sources-c9 writes the JavaScript side, and both sides agreed the format on 2026-11-10.

| File | Script | Holds |
|:--|:--|:--|
| `unicode-sets.json` | `oracle/unicode-tables.ts` | 73 sets, about 16,500 ranges, in the order `unicode.rs` lists them |
| `case-folds.json` | `oracle/case-folds.ts` | 1,427 orbits of more than one character, and 104 folds to a string |
| `classes.json` | `oracle/classes.ts` | the five sets read by tokenizing |

### 5.2 The format

- `JSON.stringify(value, null, 2)` and a trailing newline, keys in a fixed order.
- Every file has `"measured"`, then `"script"`, the script that wrote it, then the data.
  `"measured"` takes the Shiki version from `package.json`:
  - `"Oniguruma, as shiki 4.6.0 bundles it"` for `unicode-sets.json` and `case-folds.json`;
  - `"shiki 4.6.0, tokenizing with its json, typescript and python grammars"` for `classes.json`.
- `oracle/facts.ts` holds each file's type: `UnicodeSetsFacts`, `CaseFoldsFacts`, `ClassesFacts` and `SetFact`.
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
- `classes.json` adds `points` before `sets`: the code points `oracle/classes.ts` probes.
  They are `["0080..D7FF", "E000..33FFF", "E0000..E01FF"]`.
  The writer limits each `unicode-sets.json` set to those points to build the other twelve tables of `src/classes/table.rs`.
  readable-sources-c9's review rebuilt all twelve that way, byte for byte.
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

### 5.3 The checks

- A fact file against its script: `check:unicode-tables`, `check:case-folds` and `check:classes` run the script and fail when the file changes.
  They run in the Generated tables workflow, which needs Shiki's packages.
- A generated file against its facts and grammars: `mise run codegen -- --check`, in `verify`.
- The header of each generated file names the command, the writer module and each input, so a reader finds both ends.

## 6. Writing the code

### 6.1 Two writers

| Output | Writer | Why |
|:--|:--|:--|
| Structured code: enums, `match` arms, rule tables, start tables | `quote`, laid out by `prettyplease` | syn and rust-analyzer build code this way, by the [research][research] |
| Numeric tables: Unicode ranges, folds, word blobs and word tables | `numbers.rs`, which fills each line up to 100 columns | prettyplease fills only arrays of plain literals ([`simple_array`][prettyplease-array]) |

- prettyplease's output depends only on its version.
  rustfmt's output depends on the toolchain, so the system rustfmt never lays out a committed file.
  ICU4X's CI notes that generated Rust is not stable across Rust versions ([research][research], "Where code generation runs").
- prettyplease breaks at its own margin, 89 columns, and writes one entry per line past it.
  The lines rustfmt gave up on today, up to 4,380 columns in C++, C# and Swift, are laid out by it.

### 6.2 Headers, formatting and lints

- Each file opens with `// @generated by <writer> from <input>.` and `` // Regenerate with `mise run codegen`; do not edit. ``, as unicode-ident's does, then its `//!` docs.
- Its `mod` line carries `#[rustfmt::skip]`, as [unicode-ident's `lib.rs`][unicode-ident-write] does.
  The `@generated` marker alone does not stop rustfmt: `format_generated_files` is unstable and defaults to `true` ([rustfmt configuration][rustfmt-config]).
- A lint the generated code trips gets `#[expect(lint, reason = "…")]` on the smallest item, as [`CODING_STANDARDS.md`][standards] 3.3 asks.
- Every map the writers walk is a `BTreeMap`, or the writer sorts it first, so a run writes the same bytes every time.

## 7. Order

The five changes land in this order:

1. [grammar-to-rust][grammar-to-rust]: the `Cow` model, the crate-visible regex parser, then the `codegen` and `format` features.
2. [unstable-rule-engine][unstable-rule-engine]: `nishiki::unstable` and its cfg setup.
3. This change's word tables: the pull request that moves the 7 word scripts into `codegen/` creates the crate as this design describes.
   - It moved ahead of [rule-table-codegen][rule-table-codegen] task 1.1.
     The word tables need only the grammar model and the regex parser under `nishiki::unstable`, and no rule table.
   - So the crate, its tasks and its checks land with one small writer, before the rule tables depend on them.
   - `fence-stops.ts` follows in its own pull request.
4. [default-matchers][default-matchers]: the prototype's steps, measured until each language reaches the hand matchers.
5. [rule-table-codegen][rule-table-codegen]: `codegen/` writes Protocol Buffers' tables, then one family at a time.
   - Each move first adds its grammars to `tests/engine/grammars/` through `oracle/engine-grammars.ts`, with their licences in `oracle/licences.ts`.
   - The last family deletes `oracle/rule-generator.ts`, `oracle/rule-grammar.ts`, `oracle/php-family-grammar.ts` and `check:rule-grammars`.
6. The rest of this change:
   - `AGENTS.md`, `CODING_STANDARDS.md` and `oracle/AGENTS.md` gain the rules of the proposal and how to move one language;
   - Unicode sets, case folds and classes are written from readable-sources-c9's fact files.

[cargo-build-scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[cargo-rustflags]: https://doc.rust-lang.org/cargo/reference/config.html#buildrustflags
[default-matchers]: ../default-matchers/proposal.md
[grammar-to-rust]: ../grammar-to-rust/proposal.md
[prettyplease-array]: https://github.com/dtolnay/prettyplease/blob/49e50897b580a5b7591ead6ef73f3fe09e76d896/src/expr.rs
[proposal]: proposal.md
[ra-codegen]: https://github.com/rust-lang/rust-analyzer/blob/2a362a41a8eb6e090f1a12d9b712df85af6f0c33/xtask/src/codegen.rs
[research]: ../../research/2026-11-10-rust-codegen-practice.md
[rule-table-codegen]: ../rule-table-codegen/proposal.md
[rustfmt-config]: https://github.com/rust-lang/rustfmt/blob/main/Configurations.md#format_generated_files
[standards]: ../../../CODING_STANDARDS.md
[syn-codegen]: https://github.com/dtolnay/syn/blob/97ec37dda8a893a460066da694fd1eb7cb30538a/codegen/src/file.rs
[unicode-ident-write]: https://github.com/dtolnay/unicode-ident/blob/28018f288934fa6be60a6e791e071b7a7195643e/generate/src/write.rs
[unstable-rule-engine]: ../unstable-rule-engine/proposal.md
