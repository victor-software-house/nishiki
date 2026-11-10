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
  The rule engine calls it `Payload` today, and [unstable-rule-engine][unstable-rule-engine] task 1.1 renames it when it becomes public.

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
- `codegen --compare` reports a differing set by its pattern.
  The pull request lists it, and Identity and Callgrind check it.
  The `generated-rust` spec allows such a set and no other token change.
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
`codegen --compare` maps each old name to its new one by pattern id, and compares the tokens after that.

The first Swift move proves the rule:

- its pull request renames Swift's matchers and their hand-written overrides;
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
- a `names` entry names a pattern the grammar lacks, breaks rule 3.9, or passes 40 characters (section 2.4).

Each variant names the pattern id or include that failed.
The stable surface of [grammar-to-rust][grammar-to-rust] cannot fail.

## 4. The proof

- `mise run codegen -- --check` passes in `verify`.
- Each move is checked against its base commit in a build box, `8x16` or smaller, `--ttl 2h`, ephemeral:
  - `codegen --compare <base>` parses each changed file on both sides with `syn` and compares the tokens.
    Layout and `//` comments differ; tokens, doc comments included, do not.
  - Identity and `check:features` pass.
- A changed start set (section 2.3) is listed in the pull request, with its Callgrind rows.
- The output of the compare goes in the pull request body, under "How it was verified".
- `check:rule-grammars` runs the remaining scripts until the last one goes, then it is deleted.

[default-matchers]: ../default-matchers/proposal.md
[grammar-to-rust]: ../grammar-to-rust/proposal.md
[proposal]: proposal.md
[rule-generator]: ../../../oracle/rule-generator.ts
[rule-grammar]: ../../../oracle/rule-grammar.ts
[unstable-rule-engine]: ../unstable-rule-engine/proposal.md
