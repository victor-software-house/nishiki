# Design

The engine's hand-off on main (approved by the lead, 2026-11-04).

Sections 1, 2, 8 and 9 of the [`halt-contract` design][contract], made concrete for main after [#228][pr228] and [#325][pr325], with HAML as the first user.
The `halt-contract` change's tasks for `Halts` and entries stay deferred; this needs neither.

## 1. What HAML needs

HAML's includes of hand-lexed grammars fall into two shapes (Shiki's `haml` grammar, 4.4.3):

| Shape | Rules | Where the embedding rule's end can match |
|:--|:--|:--|
| Filters | `:ruby`, `:javascript`, `:css`, `:markdown` (`^(\s*):name`, end `^(?!\1\s+\|$\n*)`) | only at a line's start |
| Inline | `rubyline` (`= expr`, end `…$\|$`), `#{…}` through `nest_curly_and_self` | anywhere on the line |

1. A filter's end needs the indentation its begin captured, so the halt is a back-referenced pattern filled when the rule opens.
   A `fn(&[u8], usize) -> bool` cannot hold it.
2. A filter's end and its `before` patterns can match only at a line's start.
   The engine tries them there itself and hands over the rest of the line whole, so the target needs no halting entry: `Language::lex_embedded` serves, Markdown included.
3. An inline hand-off halts mid-line, so its target needs `lex_until`: Ruby gets `halts: true` on its row (its `lex_until` exists), and JavaScript and CSS have theirs.
4. Sass and CoffeeScript have no hand lexer.
   They stay includes compiled into HAML's table and become engine rows of their own (the [`halt-contract` design][contract], section 13.1).

## 2. A halt that carries context

`Halt` becomes a trait object instead of a function pointer:

```rust
pub(crate) trait Halt {
    /// The first place in `from..=to` where the halt holds, in `s`.
    fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize>;

    /// Whether the halt holds at `p`.
    fn holds(&self, s: &[u8], p: usize) -> bool {
        self.first(s, p, p) == Some(p)
    }
}

impl Halt for fn(&[u8], usize) -> bool { /* today's per-place loop */ }
```

1. Lexers take `&dyn Halt` where they take `Halt` today; `halt_before` calls `first`, and a lexer that tests one place, as YAML does, calls `holds`.
2. Today's halts are `fn` items, which keep working through the blanket impl: one indirect call per place, as now.
3. The engine's halt holds the places where its end and `before` rules start on the line.
   The engine finds them before the target runs (section 4 point 3).
   The halt answers `first` from them.
   The answer depends only on the bytes and the place (the [`halt-contract` design][contract], section 4, rule 1).
4. csharp's Lua halt, which needs the `]=*]` level its begin read, is the same shape: a struct holding the level.
   One change of `Halt` serves both (lead, 2026-11-04):
   - csharp makes it in its Lua cdef PR, with the `]=*]` halt as its first stateful user;
   - the engine builds on it.

   The blanket impl stays, so today's halts compile unchanged.
   No `&dyn` call lands on the plain path of a lexer that does not halt; Callgrind on the hand lexers checks it.
5. The `before`, `after` and `any` of the [`halt-contract` design][contract]'s section 4 become three `Option<&dyn Halt>` when `Halts` lands; nothing here depends on the struct.
6. csharp and the engine agreed on three points for the trait (2026-11-04):
   - Within one `lex_until` call, `s` is the whole line.
     Places count from its start.
     The bytes are the same on every call.
   - `halt_before` keeps its split.
     It tries `end` up to and at `to`, and `after` only before `to`.
     The bytes it passed stay the embedded lexer's charge.
   - The owner of a stateful halt charges the halt's own search after the call.
     The trait has no work parameter.
7. A halt holds no `RefCell`, because a second borrow could panic (lead, 2026-11-04).
   The engine's halt holds no regex scratch at all (section 4 point 3).

## 3. The table

1. A hand-off belongs to the list the include stood in.
   A rule reached through several lists has different neighbours in each list.
   - A new `Grammar::hand_offs: &'static [HandOff]` holds the hand-offs.
   - `Grammar::list_hand_offs: &'static [Option<u8>]` names each list's hand-off, by its index in `hand_offs`.
     Every step of the handing loop looks its open rule's list up there, in one load.
     A linear search of `hand_offs` cost HAML's `region` 44,800 instructions on its fixtures at one codegen unit, once HAML had nine hand-offs.
   - That list stays apart from any plain list with the same rules.
     It holds the rules before the include, then the rules after it.
     No rule stands for the include, and no scan meets one.
   - `List` is unchanged.
     A third field made it 24 bytes instead of 16.
     Indexing the lists then cost Nix's `scan_list` 6,580 instructions on its fixtures at one codegen unit.

   ```rust
   pub(crate) struct HandOff {
       /// The language that lexes the region.
       pub(crate) language: Language,
       /// The rules listed before the include. The engine tries them before
       /// the language's patterns.
       pub(crate) before: usize,
       /// The rules listed after it. The engine tries them where the language
       /// matches nothing.
       pub(crate) after: usize,
   }
   ```

2. `oracle/hand-lexed.ts` sits beside `oracle/licences.ts`.
   It lists the languages a table can hand a region to, by Shiki id, scope name, `Language` variant and Cargo feature.
   - Most of them nishiki lexes by hand.
     Sass and CoffeeScript run on the engine with tables of their own.
     A hand-off to one of them runs the engine again on the lent scratch.
   - A spec names the languages its table hands off with `>`, as in `haml>ruby>javascript>css>markdown`.
   - The generator neither loads nor compiles those grammars.
     They need no licence entry for bundling.
   - An include of one of their whole grammars becomes a hand-off.
   - An include of one of their repository rules compiles into the table as today (the [`halt-contract` design][contract], section 1.3) when the grammar is loaded.
     It is dropped when the grammar is not loaded.
3. The table does not say where the halts can start.
   The tokenizer finds them on each line before the target runs (section 4 point 3).
   A line with no halt left goes to the target whole.
4. A list has one hand-off at most.
   The generator fails on a second.
   It drops Vue's redundant second include first (the [`halt-contract` design][contract], section 2).
5. Until the tokenizer hands off (section 4), a list with a hand-off lexes as a list without the include.
   Shiki does the same for a grammar it has not loaded.
   No shipped table holds a hand-off yet.
6. A language whose table holds a hand-off needs its target's feature.
   `lang-haml` turns on `lang-ruby`, `lang-javascript`, `lang-css` and `lang-markdown`.

## 4. The tokenizer

1. A grammar that hands off runs its own instance of the loop.
   - `Halting` gains `const HANDS_OFF: bool`.
     A third type, `Handing`, carries optional halts and hands off.
     `Whole` and `Embedded` keep `HANDS_OFF = false`.
   - Every hand-off branch then compiles out of the plain and halting loops, as the halt test does ([#325][pr325]).
   - The language's row holds `hands_off: true`, and its arm calls the handing entry.
     No line tests `hand_offs.is_empty()` at run time.
   - `rebuild` and `intern` take the same parameter.
     Only a handing grammar has hand-off frames to load and store.
2. When the open rule's list has a hand-off, the engine pushes a hand-off frame (section 5).
   Each step then scans the `before` list and the end in place of the whole list.
   - When none of them starts on the rest of the line, the rest goes to `lex_embedded` whole.
     A HAML filter's lines are this case, because all its rules start with `^`.
   - Otherwise the target's `lex_until` runs to the first place where it halts, or to the line's end.
     A target without `lex_until` lexes the rest whole, as in the rule engine's embedding.
3. The engine finds the halts before the target runs.
   It does not search while the target runs.
   - It scans the `before` list and the end from the region's start.
     It scans again from one character past each match.
     The starts go into a reused buffer.
     The `after` list gets a buffer of its own.
   - The target's halt answers `first` from the sorted starts.
     It holds no regex scratch and borrows nothing from the engine.
     It cannot reach itself again.
     No scratch is lent to it, and no put-back test is needed.
   - A start inside one of the target's own contexts costs one wasted search.
     The work budget charges it.
   - A search from another place finds the same start only when the pattern ignores where the search began.
     `\A` depends on that place only on a document's first line, and a region never starts before that.
   - `\G` matches only at the anchor, where the embedding rule's begin ended on this line.
     - vscode-textmate keeps that anchor while the region's own rules open and close.
     - Hand-lexed tokens do not move it either.
     - So `\G` halts where Shiki matches it, in the `before` and `after` patterns and in the embedding rule's end.
     - Vue's nested `<template>` rule needs this: its first pattern, `tag-stuff`, begins with `\G`.
   - On a later line, `\G` matches at the line's start when the embedding rule's begin took the line feed.
     - vscode-textmate reads that flag, `beginRuleCapturedEOL`, from the innermost open rule.
     - That rule is the embedding one only while the language has its root alone open.
     - So the engine passes the flag as `Line::anchored_at_root`, apart from the `embedded` anchor a `while` rule leaves.
     - The rule engine and the engine apply it at their root only.
       In an open context, the context's own begin decides, which the rule engine tracks (`Grammar::CAPTURED_EOL`).
4. The step calls the target in place, in the cold `Tokenizer::run_language`.
   The step then goes on where the target halted.
   - At a halt the engine runs its own step.
     The end pops the rule together with the hand-off frame.
     A `before` rule matches and applies.
     The region then resumes from the target's state.
   - An earlier shape unwound the loop to the line's entry at each hand-off.
     It could not hand off inside a capture, which runs nested in a step.
5. A capture whose rule includes a hand-lexed grammar hands its text off too.
   CoffeeScript's backticks, `` (`)(.*)(`) ``, hold JavaScript this way.
   - The capture's run ends at the capture's end.
     The target sees the line cut there.
     vscode-textmate also lexes a capture as the line cut at its end, `lineText.substring(0, captureEnd)`.
     So a target that cannot halt still stops at the capture's end.
   - The capture's hand-off frame closes with the capture.
   - Only a capture whose rule can reach a hand-off runs the handing loop.
     Every other capture runs the plain loop, which costs less per step.
   - The generator lists those capture rules in `Grammar::handing_captures`, counting hand-offs in a rule nested in the capture's rule too.
     PHP's strings reach SQL that way.
     A test over every table checks the list.
6. A target needs the whole crate `Scratch`.
   - `scratch.textmate` is a `Box`.
     A handing line swaps it with the spare kept inside it, `textmate::Scratch::spare: Option<Box<textmate::Scratch>>`.
     That moves two pointers per line, and nothing at a hand-off.
   - The tokenizer lexes with the engine's scratch apart from the whole one, and lends the whole one to each target.
     The line swaps them back at its end.
   - An engine fence inside a hand-lexed region then has buffers and grammar caches of its own.
     An example is PowerShell in Markdown in HAML.
     The outer line's buffers stay untouched.
   - The spare recurses.
     Each engine level keeps the spare of the level below.
     A chain of engine, hand lexer, engine, hand lexer and engine allocates one spare per engine level on its first line.
     The depth bounds the spares.
   - An allocation test checks that later lines allocate nothing.
     One test covers two engine levels: HAML, Markdown and PowerShell.
     Another covers three: HAML, Ruby, HAML, Markdown and PowerShell.
     The third level comes through Ruby's `<<~HAML` heredoc.
     Shiki's Markdown grammar has no HAML fence, so Markdown cannot give it.
7. The Callgrind gate is every engine language at or under 1.000.
   The plain instances' code is unchanged.

## 5. State and limits

1. The hand-off frame is the [`halt-contract` design][contract]'s section 8: `kind` `tag::TEXTMATE | 1`, `payload` the target's state, `stack` the embedding rule's content stack, `parent` the embedding rule's frame.
   The target is the one `HandOff` in the parent rule's list, so the frame stores no index.
2. Work: the engine lends what its line has left, in steps of the work limit, and charges what the target returns in `Finished`, times `STEP`.
3. Depth stays per language, and the target's limits go to the same `cx.report` (the [`halt-contract` design][contract], sections 9.1 and 9.3).

## 6. Order

1. `Halt` as a trait object, in csharp's Lua cdef PR, with every lexer's `lex_until` taking `&dyn Halt`, and fixtures unchanged.
2. The generator's hand-lexed list and `RuleKind::HandOff`, with tables unchanged for every grammar that includes no hand-lexed one.
3. The tokenizer's hand-off, the frame and the resume.
   They are tested on a test grammar that hands off to JavaScript inline and to Markdown by whole lines.
4. HAML on it, with Sass and CoffeeScript as engine rows, each after its licence is read at the source.

[pr228]: https://github.com/victor-software-house/nishiki-archive/pull/228
[pr325]: https://github.com/victor-software-house/nishiki-archive/pull/325
[contract]: ../halt-contract/design.md
