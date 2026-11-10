## Purpose

Let a grammar that nishiki's TextMate engine runs hand an embedded region to the hand lexer of the embedded language.
Let the engine take the line back where the grammar ends the region.

## ADDED Requirements

### Requirement: An embedded hand-lexed language runs on the hand lexer

A rule the engine opens can include a whole grammar that nishiki lexes by hand.
The engine SHALL then lex the rule's contents with that language's hand lexer, starting it on the rule's content stack.

#### Scenario: Julia embeds Python

- **WHEN** the engine lexes `py"""` followed by Python code in a Julia document
- **THEN** the Python lexer gives the code its tokens, on the stack of the macro's contents, and the scopes match Shiki's

### Requirement: The engine takes the line back at the region's end

The engine SHALL resume its own rules at the first of these matches made while only the embedded language's root context is open:

- the embedding rule's end;
- a pattern listed before the include.

#### Scenario: The closing quotes end the region

- **WHEN** a Python region embedded by `py"""` reaches `"""`
- **THEN** the quotes get the Julia rule's end scopes and the rest of the line is Julia

### Requirement: The embedded state survives the line

The engine's state at the end of a line inside an embedded region SHALL hold the embedded language's state.
The next line then resumes the embedded lexer where it stopped.

#### Scenario: A region spans lines

- **WHEN** an embedded region opens on one line and closes three lines later
- **THEN** each line in between continues the embedded language's state, and equal states give equal engine states

### Requirement: A hand-off grammar is not compiled into the table

The table generator SHALL compile no rule of a hand-lexed grammar into a table that hands off to it.

#### Scenario: Julia's table after the change

- **WHEN** Julia's table is generated
- **THEN** it holds no rule of the Python, JavaScript or SQL grammars
