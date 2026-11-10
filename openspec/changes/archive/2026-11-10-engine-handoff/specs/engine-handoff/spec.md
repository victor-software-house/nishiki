## Purpose

Let a grammar that nishiki's TextMate engine runs hand an embedded region to the hand lexer of the embedded language.
Let the engine take the line back where the grammar ends the region.

## ADDED Requirements

### Requirement: An embedded hand-lexed language runs on the hand lexer

A rule the engine opens can include a whole grammar that nishiki lexes by hand.
The engine SHALL then lex the rule's contents with that language's hand lexer, starting it on the rule's content stack.

#### Scenario: A caller's grammar embeds JavaScript

- **WHEN** the engine lexes `<script>` followed by JavaScript in a grammar that includes `source.js` without loading it
- **THEN** the JavaScript lexer gives the code its tokens, on the stack of the rule's contents

### Requirement: The engine takes the line back at the region's end

The engine SHALL resume its own rules at the first of these matches made while only the embedded language's root context is open:

- the embedding rule's end;
- a pattern listed before the include.

#### Scenario: The end tag ends the region

- **WHEN** a JavaScript region embedded by `<script>` reaches `</script>` on the same line
- **THEN** the end tag gets the embedding grammar's scopes and the rest of the line is that grammar's

### Requirement: The embedded state survives the line

The engine's state at the end of a line inside an embedded region SHALL hold the embedded language's state.
The next line then resumes the embedded lexer where it stopped.

#### Scenario: A region spans lines

- **WHEN** an embedded region opens on one line and closes three lines later
- **THEN** each line in between continues the embedded language's state, and equal states give equal engine states

### Requirement: A hand-off grammar is not compiled into the table

The compiler SHALL compile no rule of a hand-lexed grammar into a table that hands off to it.

#### Scenario: A caller's grammar includes CSS

- **WHEN** a caller links a grammar whose `<style>` rule includes `source.css`, with no CSS grammar in the set
- **THEN** the tables hold no rule of the CSS grammar, and the `<style>` rule's list hands off to the CSS lexer
