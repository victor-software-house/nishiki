# Proposal

## Motivation

nishiki's TextMate engine runs a grammar's rules.
A grammar that embeds another language includes that language's grammar.
Today the engine can follow such an include only into an engine copy of the embedded grammar.
That copy is compiled into the same table.
This blocks or degrades every grammar that embeds a language nishiki already lexes by hand:

1. Several grammars wait for this change ([licence gate][gate], operator decisions of 2026-11-03):
   - Perl and Elixir include HTML;
   - Objective-C, PHP, Vue and Svelte embed C, HTML, CSS and JavaScript.
2. Julia's table holds engine copies of the Python, JavaScript and SQL grammars.
   Hand lexers for all three are already in the crate.
   The copies are part of the 707,720 bytes Julia adds to a release build (`feat/textmate-julia`, measured 2026-11-03).
   The copies' own share was not measured.
3. Sixty-one of Shiki 4.6.0's grammars import another grammar.
   None of them can reach the hand lexers.

The hand lexers already embed one another the other way round:

- HTML runs CSS and JavaScript;
- GraphQL runs JavaScript;
- ERB (in review) runs Ruby.

Each does it through the embedded language's `lex_until` and a `Halt` ([`src/language.rs`][language]).
This change lets the engine be the embedding side of that same contract.
The [`halt-contract` change][contract] widens the contract itself: `Halts`, entries and one entry point for every language.

## Changes

- The table generator marks an include of a hand-lexed grammar as a hand-off to that language and entry.
  It no longer compiles the grammar's rules into the table.
- The engine starts the embedded language on the rule's content stack.
  It keeps the embedded state in a frame of its own.
  It takes the line back where the embedded lexer halts.
- Depth stays counted per language.
  A pinned depth therefore keeps its output.
- The languages on a line share its work budget once core-fix's shared line budget lands.

## Capabilities

### New Capabilities

- `engine-handoff`: a grammar the engine runs hands an embedded region to the hand lexer of the embedded language.
  It takes the line back where the grammar's end matches.

### Modified Capabilities

None.
No `textmate-engine` requirement covers how an include is compiled, so the new capability changes none of them.

## Impact

- `src/textmate/`: the hand-off frame, the halt built from the end pattern, the resume.
- `oracle/engine-tables.ts`: the hand-off entry in the tables.
- Julia's table loses its engine copies of Python, JavaScript and SQL.

[gate]: https://github.com/victor-software-house/nishiki-archive/blob/8f346d8e/oracle/licences.ts
[contract]: ../../halt-contract/proposal.md
[language]: https://github.com/victor-software-house/nishiki-archive/blob/8f346d8e/src/language.rs
