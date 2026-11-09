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

### Requirement: An injection halts at any depth

A language embedded with an `any` halt SHALL stop where the halt holds and none of its current context's patterns match.
This SHALL hold at any depth outside its comments.
The language SHALL return a state from which it resumes inside the same contexts.

#### Scenario: ERB inside a JavaScript string

- **WHEN** `<%= x %>` appears inside a JavaScript string embedded in ERB
- **THEN** the JavaScript lexer halts at `<%=`, ERB lexes the tag, and the string resumes after `%>`

#### Scenario: ERB inside a JavaScript string inside HTML

- **WHEN** ERB embeds HTML, HTML embeds JavaScript, and `<%= x %>` sits in a JavaScript string
- **THEN** HTML forwards ERB's `any` to JavaScript, JavaScript halts at `<%=`, and the string resumes after `%>`

#### Scenario: ERB next to a JavaScript operator

- **WHEN** `a <%= x %>` appears in JavaScript code, where `<` is an operator
- **THEN** the operator wins at `<`, as in Shiki

### Requirement: An embedded language can start at an entry

The engine SHALL hand off to a repository rule or document mode that the embedded language offers as an entry, starting the language there.

#### Scenario: A Vue interpolation

- **WHEN** a Vue rule hands its contents to `source.ts#expression`
- **THEN** the TypeScript lexer starts in its expression context and the scopes match Shiki's

### Requirement: One entry point for embedding

Every language SHALL be lexable up to a set of halts through `Language::lex_until`, which returns its next state and where it halted.

#### Scenario: Any language embeds another

- **WHEN** any language lexes another up to an end
- **THEN** it calls `Language::lex_until` and resumes at the returned place

### Requirement: A hand-off grammar is not compiled into the table

The table generator SHALL compile no rule of a hand-lexed grammar into a table that hands off to it.

#### Scenario: Julia's table after the change

- **WHEN** Julia's table is generated
- **THEN** it holds no rule of the Python, JavaScript or SQL grammars
