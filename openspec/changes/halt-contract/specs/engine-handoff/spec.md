## ADDED Requirements

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
