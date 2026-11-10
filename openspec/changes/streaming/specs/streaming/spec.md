## ADDED Requirements

### Requirement: A document's final tokens equal lexing the whole text

Once a `Document` is finished and lexed to its end, each line's tokens SHALL equal those `Highlighter::line` gives that line.
Here `Highlighter::line` lexes the whole text line by line from `State::default`.
This SHALL hold whatever deltas and edits built the text.

#### Scenario: A fixture streamed in token-sized deltas

- **WHEN** an oracle fixture is pushed in token-sized deltas, then finished and lexed to its end
- **THEN** every line's tokens equal those of lexing the fixture line by line

#### Scenario: A document built by edits

- **WHEN** a document is built by a sequence of `replace` calls, then lexed to its end
- **THEN** every line's tokens equal those of lexing its final text line by line

### Requirement: A delta may end inside a UTF-8 character

`Document::push` SHALL accept bytes that end inside a UTF-8 character, and SHALL keep them apart from the text until the character completes.
An invalid byte sequence SHALL become U+FFFD.

#### Scenario: A character split across two deltas

- **WHEN** the bytes of `é` arrive in two deltas
- **THEN** the text holds `é` once, and no line holds a partial character

#### Scenario: An invalid sequence

- **WHEN** a delta holds a byte that cannot start or continue a UTF-8 character
- **THEN** the text holds U+FFFD in its place

### Requirement: A completed line is lexed once

Each line SHALL be lexed once when its line ending arrives, and SHALL NOT be lexed again unless an edit reaches it.
A line whose text did not change since it was last lexed as the provisional line SHALL keep those tokens without being lexed again.

#### Scenario: Lexing is counted on a streamed fixture

- **WHEN** a fixture is streamed in token-sized deltas
- **THEN** every line other than the provisional one is lexed at most once

### Requirement: Only the provisional line is lexed again on a delta

A delta that adds no line ending SHALL lex no line but the provisional one.

#### Scenario: A delta inside a line

- **WHEN** a delta adds characters and no line ending
- **THEN** only the provisional line is lexed

### Requirement: The provisional line marks the last token

While a document is not finished, its last line SHALL be provisional.
Its tokens SHALL be those of the line lexed as if it ended there.
Its `Freshness` SHALL give the byte where its last token starts.

#### Scenario: A keyword still being written

- **WHEN** the provisional line holds `in` and the next delta adds `t`
- **THEN** each render gives the line's tokens and the start of its last token

#### Scenario: Finishing the stream

- **WHEN** `finish` is called
- **THEN** the last line becomes exact

### Requirement: A long provisional line is lexed again in bounded steps

The caller can set a length and a share.
Past that length, the provisional line SHALL be lexed again only once it has grown by that share.
The caller MAY turn this off, so the line is lexed again on every delta.

#### Scenario: A long line written in small deltas

- **WHEN** one line of 100 KiB arrives in deltas of four bytes, with the default setting
- **THEN** the bytes lexed for that line stay within a fixed multiple of its length past its first KiB

### Requirement: An edit lexes until the states match

After a `replace`, lexing SHALL run from the first changed line through the new lines.
Past them, it SHALL stop at the first line whose end state equals the end state cached for the same text before the edit.

#### Scenario: An edit inside a function body

- **WHEN** an edit changes one line inside a function and leaves its end state unchanged
- **THEN** only that line is lexed

#### Scenario: An edit that opens a comment

- **WHEN** an edit opens a block comment that later text closes
- **THEN** lexing runs until the end states match again, and every line after keeps its tokens

### Requirement: Lexing goes as far as the caller requests

Mutations SHALL mark lines stale without lexing them.
`lex` SHALL lex stale lines in order up to the line the caller names, or to the end.
A stale line SHALL keep the tokens it had, and its `Freshness` SHALL mark it stale.

#### Scenario: A viewport lexed first

- **WHEN** a whole file arrives in one delta and the caller lexes to line 60
- **THEN** lines up to 60 are exact, and the rest are stale until the next `lex`

### Requirement: The document reports what changed

`take_changes` SHALL return one span of lines, before and after, that holds every line whose text or tokens changed since the last call.
It SHALL return nothing when no line changed.

#### Scenario: Several deltas between two renders

- **WHEN** three deltas arrive between two calls
- **THEN** the second call returns one span that holds every line the three deltas changed

### Requirement: Edits never panic

`replace` SHALL return an error for a range past the end of the text or off a character boundary, and SHALL leave the document unchanged.

#### Scenario: A range inside a character

- **WHEN** a range starts inside a multi-byte character
- **THEN** `replace` returns an error and the text is unchanged
