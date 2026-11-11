# Design

## Context

[proposal.md](proposal.md) states why.
The measurements are in the research note [2026-11-05-streaming][note].

What holds today, at main [68620d9e][main]:

- [`Highlighter::line`][line] lexes one line, without its ending, from a `State`, and leaves the next `State`.
- A `State` is a `u32` handle into the highlighter's arena, so two states compare in one step.
- The rule and shell engines keep the frames of the last line for the next one.
  They do so when the next line starts from the state the last one left.
  Otherwise they rebuild the frames from the arena.
  [`resume_check`][resume] proves both paths give the same tokens on every oracle fixture.
- So lines can be lexed out of order, again, or from any cached state.
  Only the cost differs.

How the other libraries stream, read in their published packages on 2026-11-05:

| Library | API | The line being written |
|:--|:--|:--|
| Shiki, [`@shikijs/stream@4.6.0`][shiki-stream] | `ShikiStreamTokenizer.enqueue(chunk)` returns stable tokens, unstable tokens and a recall count | lexed as if it ended, on every chunk |
| Pierre, `@pierre/highlights@0.4.0` | `StreamTokenizer.pushCode(chunk)` returns completed lines | held back until its line ending |
| Pierre, `@pierre/highlights@0.4.0` | `LiveTokenizer.applyEdits(edits, { renderRange })` | an edit like any other; lexing stops when the outgoing state matches the old one |

Pierre's `LiveTokenizer` lexes the `renderRange` first within a 1 ms budget, and delivers the rest later.
Lines not yet lexed keep their previous tokens.

### A later line never changes an earlier one

In Shiki's model, a line's tokens depend only on the state before it and its own text.
A line that arrives later cannot change them.

- Shiki lexes a document [in one loop over its lines][shiki-loop].
  Each pass calls `grammar.tokenizeLine2(line, stateStack)` and hands the returned `ruleStack` to the next line.
- vscode-textmate's [`tokenizeLine2`][vt-api] takes one line's text and the state before it, nothing more.
  It [appends `\n`][vt-newline] to that line, so a pattern can see the line's own end, never the next line.
- Markdown's setext heading shows it.
  The [`heading-setext` rule][setext] matches `^(={3,})(?=[\t ]*$\n?)`, so it scopes only the `===` line.
  The title above keeps `meta.paragraph.markdown`, though CommonMark renders it as a heading.
  Shiki 4.6.0 gives exactly that for `Title` then `===` (run locally, 2026-11-05).
- nishiki keeps the same contract: [`Highlighter::line`][line] takes one line and a `State`.
  Markdown picks a fence's language on the fence's opening line, so its hand-off runs forward too.

Two things can change tokens already shown:

- Text later in the same line.
  A pattern's lookahead reaches the line's end, as `int` recolours `in`.
  So the line still being written is provisional (decision 5).
- An edit, the one exception across lines.
  Replacing text in line K changes the state after K, so lines after K may change, never lines before it.
  Decision 7 lexes forward from K until the states match.

## Goals / Non-Goals

**Goals:**

- One type that serves model replies, tool-written files and edits.
- A reply with code fences is one Markdown document.
  Markdown already hands each fence to its language's lexer.
- Per delta, work bounded by the line being written, plus each newly completed line once.
- Final tokens equal to lexing the whole text with `Highlighter::line`, on every language.

**Non-Goals:**

- A speculative pass at the viewport (decision 6).
- A WebAssembly or N-API binding (the [`document-styles` design][styles-design] lists what one needs).
- Threads or background work inside the crate.

## Decisions

### 1. One type, `Document`

`Document` owns one `Highlighter`, the text, and per line its end byte, its end `State`, its tokens and its `LineReport`.

- It serves appends and edits alike, as Pierre's `LiveTokenizer` does.
  A stream is a document whose edits all land at the end.
- The name matches what it holds.
  `Stream` would mislead for edits.
- Storage is flat: the text in one `String`, the tokens of every line in one `Vec<Token>`, and one `Vec` of line entries.
  - Each entry names its line's run of tokens: where it starts and how long it is.
  - An append grows each buffer, amortized, so no line allocates.
  - A line lexed again appends a new run, and its old run becomes garbage.
    The buffer packs itself once garbage outweighs the tokens in use, so lexing one line never shifts another's tokens.
    A cumulative end per line would shift every later entry, which makes a large paste quadratic.
  - An edit splices the text and the entries, which moves the bytes after it.
    On a 1 MiB document that is about a megabyte of copying.
    A gap buffer or a rope waits until a trace shows the splice in the per-edit time.
- Token offsets stay line-relative, as `Highlighter::line` gives them.
  An edit then shifts only line entries, never tokens.

Alternative: a per-line `Vec<Token>`.
It makes edits cheap, but allocates per line on every append.
Rejected by the no-allocation rule for the hot path.

### 2. The API

```rust
pub struct Document { /* private */ }

impl Document {
    pub fn new(language: Language, theme: Theme) -> Document;
    pub fn new_with_limits(language: Language, theme: Theme, limits: Limits) -> Document;

    /// Appends bytes that may end inside a UTF-8 character.
    pub fn push(&mut self, delta: &[u8]);
    /// Appends text.
    pub fn push_str(&mut self, delta: &str);
    /// Replaces a byte range of the text.
    pub fn replace(&mut self, range: Range<usize>, text: &str) -> Result<(), EditError>;
    /// Ends the stream: the last line becomes exact.
    pub fn finish(&mut self);

    /// Lexes every stale line up to `until`.
    pub fn lex(&mut self, until: Until);
    /// The lines whose text or tokens changed since the last call.
    pub fn take_changes(&mut self) -> Option<Changes>;

    pub fn text(&self) -> &str;
    pub fn line_count(&self) -> usize;
    pub fn line(&self, index: usize) -> Option<DocumentLine<'_>>;
    pub fn highlighter(&self) -> &Highlighter;

    /// Sets when a long provisional line is lexed again (decision 4).
    pub fn set_tail(&mut self, tail: Tail);
    /// Starts a fresh highlighter and marks every line stale (decision 8).
    pub fn reset(&mut self);
    /// Switches the theme and restyles every token (`document-styles`, decision 1).
    pub fn set_theme(&mut self, theme: Theme);
}

pub enum Until { Line(usize), End }

pub enum Tail {
    /// Lex the provisional line again on every delta.
    EveryDelta,
    /// Past `short` bytes, lex it again once it grew by `1 / growth` since its last lexing.
    Amortized { short: usize, growth: usize },
}

pub struct Changes { pub start: usize, pub old_end: usize, pub new_end: usize }

pub struct DocumentLine<'a> { /* text, tokens, kind, report */ }

impl DocumentLine<'_> {
    /// Each token's end and style, from the per-stack cache (`document-styles`, decision 1).
    pub fn styled(&self) -> impl Iterator<Item = (usize, Style)> + '_;
}

pub enum Freshness {
    /// Lexed from the exact state before it; final unless an edit reaches it.
    Exact,
    /// The last line of an open stream; `stable` is the byte where its last token starts.
    Provisional { stable: usize },
    /// Not lexed since an edit before it; its tokens are the ones it had, or none.
    Stale,
}

pub enum EditError { OutOfBounds, NotCharBoundary }
```

- Mutations change the text and mark lines stale.
  `lex` does the lexing, so a caller can lex its viewport first and the rest later.
  A caller with no viewport calls `lex(Until::End)` after each mutation.
- `take_changes` merges every change since the last call into one span, given as line numbers before and after.
  A renderer calls it once a frame, whether one delta or ten arrived.
  The span is conservative: it may include lines that ended up unchanged.
- `Freshness` is an enum, not flags, so a renderer matches every case.
- Offsets are bytes.
  `replace` returns an error, never panics, on a range past the end or off a character boundary.
- `push` keeps up to three bytes of an unfinished UTF-8 character apart from the text.
  An invalid sequence becomes U+FFFD, as `String::from_utf8_lossy` and a streaming `TextDecoder` do.
- Line endings follow Shiki's split.
  `\n` ends a line, and a `\r` before it belongs to the ending.
  A provisional line that ends in `\r` is lexed without it until the next byte shows whether a `\n` follows.

### 3. A completed line is lexed once

On `\n`, the completed line is lexed from the state the line above left.

- If the provisional line's text has not changed since its last lexing, its tokens and end state are already the final ones.
  `Highlighter::line` excludes the line ending, so an ended line lexes as the same text unended did.
  The tokens move from the tail buffer into the line buffer, and nothing is lexed.
- A completed line is never lexed again unless an edit reaches it.
- A test counts the calls to `Highlighter::line` over a streamed fixture: one per line, plus the tail's.

### 4. Only the tail is lexed again on each delta

The provisional line is lexed from its start on every delta, into a reused tail buffer.

- On the traces, one delta is about 3.1 characters ([note][note], deltas), so a line of 40 characters is lexed about 13 times.
  At the steady per-line cost of 0.1 to 3 µs at p50, that is the per-delta cost.
- A line that grows long, such as minified JSON written in one line, would make that quadratic.
- Above 1 KiB, the tail is lexed again only once it has grown by an eighth since its last lexing.
  Past its first KiB, the work on one line stays within nine times its length.
  The bytes past the last lexing show as held back.
- The default is `Tail::Amortized { short: 1024, growth: 8 }`.
  `Document::set_tail` changes it, and `Tail::EveryDelta` turns the rule off (root guide, Limits rule 1).
  They count bytes, never time, so the same input gives the same renders on every machine.

### 5. The last token of the provisional line is held back

The flicker measurement decides it ([note][note], section 1):

| Mode | Flickers per KiB | Characters held back per render |
|:--|--:|--:|
| As if the line ended | 119.68 | 0 |
| Last token held back | 4.18 | 18.09 |
| Last two tokens held back | 2.59 | 20.81 |
| Whole line held back | 0 | 29.16 |

- `Document` gives every token of the provisional line, lexed as if the line ended.
- `Freshness::Provisional { stable }` marks where the last token starts.
  The crate docs advise showing the bytes from `stable` in the default colour.
- Holding back one token removes 96.5% of the flicker and keeps 18 characters uncoloured.
  A second token removes a little more, at three more characters.
  A renderer that wants it has every token and can hold back more.
- The remaining flicker comes from grammars that look ahead within a line: TypeScript, HTML and C++.
  No choice of held-back tokens removes it short of the whole line.

### 6. One large delta: the exact sweep, viewport first, no speculation

When a tool writes a whole file in one delta, the caller lexes to its viewport, renders, then lexes the rest.

- A speculative pass, lexing from the initial state at the viewport, coloured the screen exactly in 60% of 140 viewports.
  It coloured under half the screen right in 6% of them, inside docstrings, heredocs and comment blocks ([note][note], section 2).
- It cannot tell when it is right.
  Its state agreed with the exact one within a screen in only half the viewports, so the exact sweep must redo the work anyway.
- In one 60 Hz frame, the exact sweep lexes 124 KiB in C++, the slowest language.
  It lexes 409 KiB to 2.9 MiB in the other languages measured.
  Speculation could only pay for text above those sizes before the viewport.
- So nishiki does not speculate.
  A viewport past those sizes shows its lines stale or uncoloured for a few frames.
- Revisit when a recorded trace has one delta whose sweep to the viewport exceeds a frame on the reference host.

### 7. Edits stop where the states match

Each line records the state it was last lexed from.
`replace` gives the lines it touches new entries, which have none, and moves the lexed frontier back to the first of them.
`lex` then runs from that line:

- it lexes each line that has no recorded state, or whose recorded state differs from the state before it now;
- it skips each line whose recorded state equals the state before it, and keeps its tokens.

So past the edit, the first line whose state matches stops the lexing, and every unchanged line after it is only compared.
The check is exact however edits and partial `lex` calls interleave.

A first version compared with the end state cached above the line instead.
After a partial `lex` and a second edit, that state could belong to a newer pass than the line below.
The stream test caught it on 18 fixtures.

Replaying 239 hunks of nishiki's own history, 238 stopped at their own last line, and one lexed two lines more ([note][note], section 3).

### 8. The arena

Each distinct state and stack is interned once in the highlighter's arena.
A long-lived document under many edits can fill it to `Limits::nodes`.

- A line that reaches the node limit is marked in its `LineReport`, as today.
- `Document::reset` starts a fresh highlighter and marks every line stale.
  A caller may call it when a report names the node limit.

## Risks / Trade-offs

- [Shiki's states substitute for nishiki's in the speculation and edit measurements] → The stream tests check nishiki's own tokens.
  They compare them with one-shot lexing on every fixture.
- [A splice on a large document costs a memory move per edit] → The edit trace times it; a gap buffer comes only if it shows.
- [`hold-last` keeps the last token uncoloured, which reads as lag on long tokens] → `stable` is advice. Every token is still given.

## Open Questions

- Whether `take_changes` should also report a span per edit for diff views.
  The single merged span is enough for every trace here.

[note]: ../../research/2026-11-05-streaming.md
[main]: https://github.com/victor-software-house/nishiki-archive/commit/68620d9ef4c7eb99a91c30f3a3d6788a2c4aaebf
[line]: https://github.com/victor-software-house/nishiki-archive/blob/68620d9ef4c7eb99a91c30f3a3d6788a2c4aaebf/src/highlighter.rs#L67-L100
[resume]: https://github.com/victor-software-house/nishiki-archive/blob/68620d9ef4c7eb99a91c30f3a3d6788a2c4aaebf/src/resume_check.rs
[shiki-stream]: https://www.npmjs.com/package/@shikijs/stream/v/4.6.0
[shiki-loop]: https://github.com/shikijs/shiki/blob/167a38fea19cfa92b760ba36e157589ca5801274/packages/primitive/src/highlight/code-to-tokens-base.ts#L164-L278
[vt-api]: https://github.com/shikijs/vscode-textmate/blob/5c67b2bf36852688c3d3d4a5e26f3b8f2d90be7f/src/index.ts#L221
[vt-newline]: https://github.com/shikijs/vscode-textmate/blob/5c67b2bf36852688c3d3d4a5e26f3b8f2d90be7f/src/grammar/grammar.ts#L374
[setext]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/packages/tm-grammars/grammars/markdown.json#L2543-L2554
[styles-design]: ../document-styles/design.md
