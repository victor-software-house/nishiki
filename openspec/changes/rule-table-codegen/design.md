# Design

This change moves the rule-engine tables of each language from its TypeScript script to `unstable::codegen`, one family at a time.
[The proposal][proposal] links the four changes split from `rust-codegen` with it.

## 1. The rule-engine scripts

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

## 2. The rule-engine tables

### 2.1 Model and parser

- `unstable::codegen` reads `compile::UnlinkedGrammar`, which `compile::compile` builds from the JSON once.
  It holds rules, includes, captures, `while`, `applyEndPatternLast` and nested repositories as typed values.
  No second JSON reader is written.
- It reads each expression with the crate's parser, `regex::parse::parse`, and its tree, `regex::ast::Node`.
  Both are `pub(crate)` since [grammar-to-rust][grammar-to-rust] task 2.1.
- Its modules each do one job:

  | Module | Job |
  |:--|:--|
  | `flatten.rs` | includes and pattern lists, with vscode-textmate's drop rule |
  | `split.rs` | a `$N` scope name into one pattern per alternative |
  | `starts.rs` | section 2.3 |
  | `tables.rs` | the typed model: `Rule`, `Pattern`, `Capture`, `ScopeName`, `CarriedState` |
  | `names.rs` | identifiers such as `KwControl` and `RootAt3` |
  | `matchers.rs` | the [default matchers][default-matchers] |
  | one writer per output file | its file |

- An act is `Match`, `Begin`, `End` or `While`, each with its own data, and every match on it is exhaustive.
- `CarriedState` is what a rule carries from line to line: nothing, the text of a begin group its end repeats, or an embedded language's state.
  [unstable-rule-engine][unstable-rule-engine] task 1.1 renamed it from `Payload` when it became public.

### 2.2 Per-language values

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
| `names` | a name for each pattern whose derived name is too long (section 2.4) | the snake-cased pattern id |
| `added_outputs` | the files a language adds, each a function of the model | the six one-off outputs |

- The values are Rust, so the compiler checks every field, and no second file format exists.
- The six one-off outputs stay with their languages: C#'s `How` table, Swift's `dispatch.rs`, and C++'s, Julia's, Svelte's and CoffeeScript's.
  Each is a function that reads the typed model and returns tokens.
  The writer holds no branch on a language's name.
- A caller's `RuleTables` is the same type, from `nishiki::unstable::codegen`.

### 2.3 The first-byte analysis

- `starts.rs` ports `startOf`, `canMatchEmpty` and `takesOneBlank` from `oracle/rule-grammar.ts`.
  It reads the parser's tree instead of the expression's text.
- It computes the same facts:
  - the first-byte set, and the places a match can start;
  - whether it can match at the end;
  - whether it can take one blank.
- The tree folds `(?i)` characters into classes of their orbits, which the TypeScript reader does not.
  A set can then gain the first byte of a character such as the Kelvin sign.
  Such a set is still sound, but it differs.
- codegen's check reports a differing set by its pattern (section 5, slice 1).
  The pull request lists it, and Identity and Callgrind check it.
  The `generated-rule-tables` spec allows such a set and no other token change.
- `oracle/rule-grammar.test.ts`'s 24 cases become unit tests with typed inputs.

### 2.4 Generated names

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
3. When no enclosing key differs, each takes its place in its pattern list as a suffix, such as `_2`.
4. A name stays within 40 characters, so a line that calls it fits in 100 columns.
   A longer name is an error (section 3), and the language's `RuleTables` gives that pattern a name in `names`.
   A `names` entry for a pattern the grammar lacks is an error too, as is one that breaks rule 3.9 or passes 40 characters.
5. Every name is unique within the language.
   After rules 1 to 4, and after `names`, a check compares all of them.
   Two equal names are an error that names both pattern ids (section 3).
   It catches a name that rule 2 or 3 changed but that still equals another pattern's name.
   It also catches a `names` entry that equals a derived name.

The same rule names every generated identifier, such as a frame's `Rule` variant.
A hand matcher takes the name of its pattern, so a move renames the hand-written function too.
Swift's port keeps its old names byte for byte (section 4).
Its shorter names follow in their own slice (section 5).
That slice proves the rule:

- it renames Swift's matchers and their hand-written overrides;
- it maps each old name to its new one by pattern id, and compares the tokens after that with `syn`;
- no generated Swift name passes 40 characters, and no line that holds one passes 100 columns.

The readability exemption for Swift's generated names ends with that pull request.

## 3. Errors

`unstable::codegen::GenerateError` is one enum, documented under `# Errors`:

- the grammar does not compile (`CompileError` as its source);
- an expression does not parse (the parser's error as its source);
- an include names no rule, and the language does not list it as external;
- a scope name repeats a group the expression lacks;
- the rules do not fit a byte, or the patterns a `u16`;
- an end repeats more than one group of its begin;
- a hand matcher names a pattern the grammar lacks;
- a derived name passes 40 characters, and `names` gives none (section 2.4);
- two patterns take the same name, derived or from `names`, and the error names both pattern ids (section 2.4);
- a `names` entry names a pattern the grammar lacks, breaks rule 3.9, or passes 40 characters (section 2.4);
- two patterns with different expressions or acts take one id, or two rules do (`IdTaken`).
  The scripts let the later one replace the earlier one there.
  The port fails instead, so a grammar that relied on the replacement fails its check.

Each variant names the pattern id or include that failed.
The stable surface of [grammar-to-rust][grammar-to-rust] cannot fail.

## 4. The proof

Every slice proves byte-identical output against its base commit (lead, 2026-11-12).

- codegen formats each file it writes with rustfmt, on the nightly `RUSTFMT_TOOLCHAIN` names, as `writeRust` does.
- A slice runs codegen and the scripts it replaces on one base.
  `git diff --exit-code` over the slice's files then shows no change.
- `mise run codegen -- --check` passes in `verify`, and Identity passes.
- Two kinds of change are allowed, each in a slice of its own, with its Callgrind rows:
  - a start set that the tree reads differently (section 2.3);
  - Swift's shorter names (section 2.4).
- `check:rule-grammars` runs the remaining scripts until the last one goes, then it is deleted.

## 5. The slices

Each slice puts its logic in `unstable::codegen` from the start (section 2.1).
The `codegen/` binary drives it, and `codegen/src/languages/` holds each language's `RuleTables` value.

1. The analyses.
   - `starts.rs` ports `startOf`, `canMatchEmpty` and `takesOneBlank` over the parser's tree.
   - The blank-run analysis moves from `codegen/src/blank_runs.rs` into `unstable::codegen` beside it.
     The `codegen/` binary keeps only the writer of `src/rules/blank_runs.rs`.
   - Both read `codegen/patterns/`.
   - `starts.rs` writes nothing yet.
     codegen's check compares its result with the tables in each `grammar.rs`, for all 48 languages.
   - Proof: no file changes, and the check lists each start set that differs.
2. The changed start sets, if slice 1 finds any.
   - codegen writes each pattern's start to `codegen/starts/<language>.json`, and each script reads its language's file.
   - A script writes its pattern file before it reads its start file.
     A new pattern then takes three steps: run the script, `mise run codegen`, then the script again.
   - PHP's, Blade's, Hack's and Julia's scripts rewrite each expression before the TypeScript reader reads it.
     They keep that reader until their family slices.
   - Proof: the diff holds only the listed sets, with Callgrind rows and Identity.
3. The model.
   - `flatten.rs` and `split.rs` build each language's pattern list from its grammar in `tests/engine/grammars/` and its `RuleTables` value.
   - codegen's check compares that list with `codegen/patterns/<language>.json`, byte for byte, for each of the 34 languages whose grammar is there.
   - Proof: no file changes, and the check passes for all 34.
   - Six pull requests add the languages, each with the variants they need (lead, 2026-11-12):
     1. the model and `flatten.rs`, for Go, Protocol Buffers, Python, Sass and Scala;
     2. `split.rs` and the names a language gives its patterns, for Java, Dart, PowerShell, VB and Jinja;
     3. includes that add nothing and where a list that reaches itself stops, for Common Lisp, GDScript and Less;
     4. includes of other grammars, for Elixir, Julia and Lua;
     5. the scripts with logic of their own, for CSS, Assembly, Swift, CoffeeScript, HAML, Ruby, Perl, SCSS, C++, C#, Bat and R;
     6. `oracle/rule-generator.ts`'s five languages, and PHP.
   - CoffeeScript and Julia moved from the second to the fourth, and CSS to the fifth (lead, 2026-11-12).
     Their scripts need those pull requests' variants.
   - The third slice split in two, and Assembly and Swift moved to the fifth (lead, 2026-11-12).
     Assembly and Swift each have logic of their own in their scripts.
   - CoffeeScript and HAML moved from the fourth to the fifth (2026-11-12).
     CoffeeScript's script drops an entry that holds only `source.js.regexp`, names capture-list rules after their key, and appends Svelte's injected patterns.
     HAML's script keys patterns by their JSON, drops a begin whose patterns are all missing, and takes a pattern from Ruby's grammar.
   - A pattern that another grammar holds is read from that grammar, never copied as text (lead, 2026-11-12).
     - A copy would go stale once task 2.5 deletes the scripts and `codegen/patterns/`.
     - HAML's tables name Ruby's escape pattern by grammar and key, and codegen passes `ruby.json` to the walk.
     - CoffeeScript's Svelte patterns follow the same rule.
     - Lua's made-up begin exists in no grammar, so its tables hold it as text.
   - Each variant of a script is a field or an enum variant in `RuleTables`, never a function per language.
   - `Reentry::WithinWalk` exists only to match the scripts' output (lead, 2026-11-12).
     Under it, one guard covers the whole walk, and a second pass lists what the guard cut short.
     A later slice may drop it once the identity fixtures show no token changes without it.
4. One slice per family: codegen writes the family's `grammar.rs`, `scope.rs` and `words.rs`.
   - The slice deletes the family's scripts, pattern files and start files.
     The blank-run analysis and `starts.rs` then read `unstable::codegen`'s own model for it.
   - A family whose grammar is missing adds it to `tests/engine/grammars/`, after its licence is read at its source.
     Astro, Kotlin, PostCSS and Svelte have no row in `oracle/licences.ts` yet.
   - Proof: section 4, on the family's files.
   - The order:
     1. Protocol Buffers (task 1.1), then the other scripts with no logic of their own (section 1);
     2. `oracle/rule-generator.ts`'s family: LaTeX, TeX, Objective-C, Objective-C++ and MATLAB;
     3. `oracle/php-family-grammar.ts`'s family: PHP, Blade and Hack;
     4. the other short scripts, a few to a slice;
     5. one slice each for C#, Astro, Perl, Swift, Svelte with CoffeeScript, C++ and Julia, with their added outputs (section 2.2);
     6. Swift's shorter names (section 2.4), after Swift's port.
        Proof: the diff holds only the renames, with Callgrind rows and Identity.
5. The last slice deletes `oracle/rule-grammar.ts`, `oracle/rule-generator.ts`, `codegen/patterns/`, `codegen/starts/` and `check:rule-grammars`.
   - Proof: no `oracle/*-grammar.ts` or `oracle/rule-generator.ts` remains, and no generated file changes.

[default-matchers]: ../default-matchers/proposal.md
[grammar-to-rust]: ../grammar-to-rust/proposal.md
[proposal]: proposal.md
[rule-generator]: ../../../oracle/rule-generator.ts
[rule-grammar]: ../../../oracle/rule-grammar.ts
[unstable-rule-engine]: ../unstable-rule-engine/proposal.md
