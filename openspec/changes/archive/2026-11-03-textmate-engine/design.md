# Design

The research note's design stands; this records what the operator's decision changes and what is still open.

## 1. Regex strategy

Two matchers behind one `Pattern`:

1. A `regex-automata` DFA for a pattern with no lookaround, backreference, `\G` or atomic group, found by the parser when it compiles the pattern.
2. nishiki's backtracking matcher for the rest, with the literal prefix, first-byte set and anchor class computed once per pattern.

The DFA is built lazily with `hybrid::dfa` so that a language costs nothing until a pattern is first searched, unless measurement shows a serialized `dense::DFA` included in the crate loads faster and fits in the crate's size.
This is the first thing to measure, not to decide.

## 2. Tables

A generator under `oracle/` reads Shiki's grammar for a language and writes Rust source into `src/textmate/data/`, one file per grammar: rules in an array, patterns as strings with their prefilters, scope names interned into the language's scope table, includes resolved to indices.
The generator is TypeScript run with Bun, as the oracle is; the crate has no build script and no JSON parser.

## 3. The tokenizer

The engine follows vscode-textmate: the end pattern first unless the rule applies it last, the leftmost match among a rule's patterns, `\G` at the end of the last begin, while or end match, back-references in end and while patterns filled when the rule opens, and the guards against endless loops.
State between lines is a hash-consed frame of (rule, interned end pattern, name stack, content stack, began-at-end-of-line), so a state is one integer as for the hand lexers.

## 4. Speed levers, in the order they are expected to pay

1. Search only the patterns that can start at or after the cursor, using first-byte sets, and keep the best match found so far so a later position does not repeat the search.
2. Memoize a pattern's next match position per line until the cursor passes it, as Oniguruma's scanner does for a set.
3. The DFA fast path.
4. No allocation per line: scratch buffers owned by the highlighter.

Each is measured on its own, and the bench table reports each.

## 5. Open

1. The compiled size per grammar against the 10 MB crate limit, with every bundled language.
2. How injections are matched cheaply: scope selectors against the current stack on every step.
3. Whether a language that later gets a hand lexer keeps the engine's tables for the second-oracle test, and where they live so the published crate does not carry them.
